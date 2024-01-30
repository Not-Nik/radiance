// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::encoding::Encoding;
use crate::error::GatewayError;
use crate::event_deserializer::EventDeserializer;
use crate::events::{EventPayload, RadianceEvent};
use eetf::{Binary, Term};
use flate2::{Compress, Compression, FlushCompress};
use futures_util::{SinkExt, StreamExt};
use serde::de::DeserializeSeed;
use std::io::Cursor;
use warp::ws::Message;

pub struct Compressor {
    compress: Compress,
    buffer: Box<[u8]>,
}

pub struct GatewayConnection {
    ws: warp::ws::WebSocket,
    compressor: Compressor,
    pub encoding: Encoding,
    compress: bool,
}

impl Compressor {
    const BUFFER_SIZE: usize = 32 * 1024;

    pub fn new() -> Compressor {
        Compressor {
            compress: Compress::new(Compression::default(), true),
            buffer: vec![0; Self::BUFFER_SIZE].into_boxed_slice(),
        }
    }

    pub fn compress(&mut self, buffer: Vec<u8>) -> Result<Vec<u8>, GatewayError> {
        let processed_pre = self.compress.total_in();

        let mut processed = 0;

        // Decompressed message. `Vec::extend_from_slice` efficiently allocates
        // only what's necessary.
        let mut compressed = Vec::new();

        loop {
            let produced_pre = self.compress.total_out();

            // Use Sync to ensure data is flushed to the buffer.
            self.compress
                .compress(&buffer[processed..], &mut self.buffer, FlushCompress::Sync)
                .map_err(|_| GatewayError::CompressionError)?;

            processed = (self.compress.total_in() - processed_pre)
                .try_into()
                .unwrap();
            let produced = (self.compress.total_out() - produced_pre)
                .try_into()
                .unwrap();

            compressed.extend_from_slice(&self.buffer[..produced]);

            // Break when message has been fully decompressed.
            if processed == buffer.len() {
                break;
            }
        }

        Ok(compressed)
    }
}

impl GatewayConnection {
    pub fn new(ws: warp::ws::WebSocket, encoding: Encoding, compress: bool) -> Self {
        GatewayConnection {
            ws,
            compressor: Compressor::new(),
            encoding,
            compress,
        }
    }

    pub async fn read_event(&mut self) -> Result<RadianceEvent, GatewayError> {
        match self.encoding {
            Encoding::Etf => self.read_event_etf().await,
            Encoding::Json => self.read_event_json().await,
        }
    }

    pub async fn send_event(&mut self, event: EventPayload) -> Result<(), GatewayError> {
        match self.encoding {
            Encoding::Etf => self.send_event_etf(event).await,
            Encoding::Json => self.send_event_json(event).await,
        }
    }

    async fn read_event_common(&mut self) -> Result<Vec<u8>, GatewayError> {
        let message = self
            .ws
            .next()
            .await
            .and_then(|r| r.ok())
            .ok_or(GatewayError::ConnectionClosed)?;

        Ok(message.into_bytes())
    }

    async fn read_event_json(&mut self) -> Result<RadianceEvent, GatewayError> {
        let json = String::from_utf8(self.read_event_common().await?)
            .map_err(|_| GatewayError::InvalidEncoding)?;

        let deserializer =
            EventDeserializer::from_json(&*json).ok_or(GatewayError::InvalidEncoding)?;
        let mut json_deserializer = serde_json::Deserializer::from_str(&*json);
        let event = deserializer.deserialize(&mut json_deserializer).unwrap();

        Ok(event)
    }

    async fn read_event_etf(&mut self) -> Result<RadianceEvent, GatewayError> {
        let etf = self.read_event_common().await?;

        let Term::Map(term) =
            Term::decode(Cursor::new(&etf)).map_err(|_| GatewayError::InvalidEncoding)?
        else {
            Err(GatewayError::IncompleteData)?
        };

        let op_term = Term::Binary(Binary::from("op".as_bytes()));
        let event_type_term = Term::Binary(Binary::from("t".as_bytes()));

        let Term::FixInteger(op) = term.map.get(&op_term).ok_or(GatewayError::IncompleteData)?
        else {
            Err(GatewayError::IncompleteData)?
        };

        let event_type_string = term.map.get(&event_type_term).and_then(|t| match t {
            Term::Atom(a) => Some(a.name.clone()),
            _ => None,
        });

        let event_type = event_type_string.as_ref().map(|s| s.as_str());

        let deserializer = EventDeserializer::new(op.value as u8, event_type);
        let etf_deserializer = serde_etf::Deserializer::from_term(Term::Map(term));
        let event = deserializer.deserialize(etf_deserializer).unwrap();

        Ok(event)
    }

    async fn send_event_common(&mut self, payload: Vec<u8>) -> Result<(), GatewayError> {
        if self.compress {
            let compressed = self.compressor.compress(payload)?;

            self.ws.send(Message::binary(compressed))
        } else {
            // fixme: unsafe
            self.ws.send(Message::text(unsafe {
                String::from_utf8_unchecked(payload)
            }))
        }
        .await
        .map_err(|_| GatewayError::ConnectionClosed)?;

        self.ws
            .flush()
            .await
            .map_err(|_| GatewayError::ConnectionClosed)?;

        Ok(())
    }

    async fn send_event_json(&mut self, event: EventPayload) -> Result<(), GatewayError> {
        let payload = serde_json::to_string(&event).map_err(|_| GatewayError::EncodeError)?;
        self.send_event_common(payload.into_bytes()).await
    }

    async fn send_event_etf(&mut self, event: EventPayload) -> Result<(), GatewayError> {
        let payload = serde_etf::to_term(&event).map_err(|_| GatewayError::EncodeError)?;
        let mut buf = Vec::new();
        payload
            .encode(&mut buf)
            .map_err(|_| GatewayError::EncodeError)?;
        self.send_event_common(buf).await
    }
}
