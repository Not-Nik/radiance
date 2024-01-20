#![feature(const_for)]
#![feature(const_mut_refs)]

mod version;

use crate::version::{Platform, Version};
use enum_map::{enum_map, EnumMap};
use once_cell::unsync::Lazy;
use std::collections::HashMap;
use std::time::SystemTime;
use warp::http::Response;
use warp::Filter;

const OSX_VERSION: Version = Version::new(0, 0, 291);
const OSX_PUB_DATE: &'static str = "2024-01-09T18:25:05";
const WIN_VERSION: Version = Version::new(0, 0, 311);
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

    // Match any request and return hello world!
    let routes = warp::any()
        .and(warp::path::full()) // Extract the full path
        .and(warp::query::<HashMap<String, String>>())
        .map(|path: warp::path::FullPath, p: HashMap<String, String>| {
            // Access the path using path.as_str()
            println!("{} {:?}", path.as_str(), p);
            Response::builder()
                .status(404)
                .header("Date", httpdate::fmt_http_date(SystemTime::now()))
                .header("Content-Type", "application/json")
                .body(String::from(r#"{"message": "404: Not Found", "code": 0}"#))
        });

    warp::serve(warp::get().or(routes))
        .tls()
        .cert_path("certs/cert.pem")
        .key_path("certs/key.pem")
        .run(([0, 0, 0, 0], 443))
        .await;
}
