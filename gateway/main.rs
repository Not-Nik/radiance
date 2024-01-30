// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod encoding;

use std::collections::HashMap;
use std::str::FromStr;
use log::debug;
use warp::Filter;
use crate::encoding::Encoding;

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
    }
}