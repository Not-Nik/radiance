// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod endpoints;
mod version;

use crate::endpoints::{forward, updates_stable};
use crate::version::{Platform, Version};
use enum_map::{enum_map, EnumMap};
use hickory_resolver::config::{ResolverConfig, ResolverOpts};
use hickory_resolver::TokioAsyncResolver;
use once_cell::unsync::Lazy;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;

const OSX_VERSION: Version = Version::new(0, 0, 295);
const OSX_PUB_DATE: &'static str = "2024-02-20T21:45:24";
const WIN_VERSION: Version = Version::new(0, 0, 311);
// This seems wrong, maybe they are using a different name for windows now, or maybe they're just lazy
const WIN_PUB_DATE: &'static str = "2021-09-22T18:16:06";
const LINUX_VERSION: Version = Version::new(0, 0, 43);
const LINUX_PUB_DATE: &'static str = "2024-02-12T21:18:51";

const VERSION_MAP: Lazy<EnumMap<Platform, (Version, &'static str)>> = Lazy::new(|| {
    enum_map! {
        Platform::OsX => (OSX_VERSION, OSX_PUB_DATE),
        Platform::Win => (WIN_VERSION, WIN_PUB_DATE),
        Platform::Linux => (LINUX_VERSION, LINUX_PUB_DATE)
    }
});

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Get discord.com's real IP, ignoring the contents of /etc/hosts
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    let response = resolver.lookup_ip("www.discord.com.").await.unwrap();

    // Collect IPs and wrap them in an Arc, so we don't have to copy for each request
    let addresses = Arc::new(
        response
            .iter()
            .map(|ip| SocketAddr::new(ip, 443))
            .collect::<Vec<SocketAddr>>(),
    );

    let updates_stable = warp::path!("api" / "updates" / "stable")
        .and(warp::query::<HashMap<String, String>>())
        .map(updates_stable);

    // Match any request and return hello world!
    let routes = warp::any()
        .and(warp::path::full()) // Extract the full path
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::any().map(move || addresses.clone()))
        .and_then(forward);

    warp::serve(warp::get().and(updates_stable).or(routes))
        .tls()
        .cert_path("certs/cert.rest.pem")
        .key_path("certs/key.rest.pem")
        .run(([0, 0, 0, 0], 4433))
        .await;
}
