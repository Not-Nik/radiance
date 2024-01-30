// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![feature(async_closure)]

mod connection;
mod encoding;
mod error;
mod events;

use crate::connection::GatewayConnection;
use crate::encoding::Encoding;
use crate::error::GatewayError;
use crate::events::IntoPayload;
use log::debug;
use std::collections::HashMap;
use std::str::FromStr;
use twilight_model::gateway::event::Event;
use twilight_model::gateway::payload::incoming::Hello;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let gateway = warp::any()
        .and(warp::ws())
        .and(warp::query::<HashMap<String, String>>())
        .map(|ws: warp::ws::Ws, p: HashMap<String, String>| {
            ws.on_upgrade(async move |ws| gateway(ws, p).await)
        });

    warp::serve(warp::get().and(gateway))
        .tls()
        .cert_path("certs/cert.gateway.pem")
        .key_path("certs/key.gateway.pem")
        .run(([0, 0, 0, 0], 4434))
        .await;
}

pub async fn gateway(ws: warp::ws::WebSocket, p: HashMap<String, String>) {
    let zlib = p
        .get("compress")
        .map(|c| c == "zlib-stream")
        .unwrap_or(false);

    if let Some(Ok(encoding)) = p
        .get("encoding")
        .map(|s| s.clone())
        .map(|s| Encoding::from_str(&*s))
    {
        debug!("Opening WebSocket ({encoding}, zlib: {zlib})");

        let connection = GatewayConnection::new(ws, encoding, zlib);

        let res = failing_gateway(connection).await;
        res.unwrap();
    }
}

async fn failing_gateway(mut connection: GatewayConnection) -> Result<(), GatewayError> {
    let mut sequence = 0;

    // fixme: should the interval be dynamically chosen?
    let hello = Event::GatewayHello(Hello {
        heartbeat_interval: 41250,
    });

    connection
        .send_event(hello.into_payload(&mut sequence))
        .await?;

    debug!("Sent hello");

    Ok(())
}
