// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::encoding::Encoding;
use crate::error::GatewayError;
use crate::events::EventPayload;
use flate2::{Compress, Compression, FlushCompress};
use futures_util::SinkExt;
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

    pub async fn send_event(&mut self, event: EventPayload) -> Result<(), GatewayError> {
        match self.encoding {
            Encoding::Etf => self.send_event_etf(event).await,
            Encoding::Json => self.send_event_json(event).await,
        }
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
