// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![feature(const_for)]
#![feature(const_mut_refs)]

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
use std::str::FromStr;
use std::sync::Arc;
use warp::Filter;

const OSX_VERSION: Version = Version::new(0, 0, 291);
const OSX_PUB_DATE: &'static str = "2024-01-09T18:25:05";
const WIN_VERSION: Version = Version::new(0, 0, 311);
// This seems wrong, maybe they are using a different name for windows now, or maybe they're just lazy
const WIN_PUB_DATE: &'static str = "2021-09-22T18:16:06";
const LINUX_VERSION: Version = Version::new(0, 0, 40);
const LINUX_PUB_DATE: &'static str = "2024-01-09T18:23:17";

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

    let mut response = resolver.lookup_ip("www.discord.com.").await.unwrap();

    // Collect IPs and wrap them in an Arc so we don't have to copy for each request
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
        .cert_path("certs/cert.pem")
        .key_path("certs/key.pem")
        .run(([0, 0, 0, 0], 443))
        .await;
}
