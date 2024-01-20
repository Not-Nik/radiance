use crate::version::{Platform, Version};
use crate::VERSION_MAP;
use reqwest::ClientBuilder;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;
use warp::http::Response;
use log::{debug, warn};

pub async fn forward(
    path: warp::path::FullPath,
    p: HashMap<String, String>,
    discord: Arc<Vec<SocketAddr>>,
) -> Result<impl warp::Reply, Infallible> {
    // Force client to resolve discord.com to external address, ignoring /etc/hosts
    let client = ClientBuilder::new()
        .resolve_to_addrs("discord.com", &*discord)
        .build()
        .unwrap();

    let path = path.as_str();

    // Forward asset files (and /app)
    if path.ends_with(".js")
        || path.ends_with(".json")
        || path.ends_with(".css")
        || path.ends_with(".png")
        || path == "/app"
    {
        debug!("Fetching {} from discord.com", path);

        let mut target_url = format!("https://discord.com{}", path);

        if !p.is_empty() {
            target_url += "?";
        }
        for (name, param) in &p {
            target_url += &*format!("{}={}", name, param);
        }
        match client.get(target_url).send().await {
            Ok(response) => {
                // reqwest doesn't return `http` Responses, so hack one together
                let mut builder = Response::builder().status(response.status());

                for (name, value) in response.headers() {
                    builder = builder.header(name.clone(), value.clone());
                }

                let body = response.text().await.unwrap_or_default();

                return Ok(builder.body(body).unwrap());
            }
            Err(e) => {
                warn!("Error making HTTPS request: {}", e);
            }
        }
    }

    // If the requested path isn't an asset or asset fetching fails

    // Access the path using path.as_str()
    warn!("Request at {} {:?} wasn't handled", path, p);
    Ok(Response::builder()
        .status(404)
        .header("Date", httpdate::fmt_http_date(SystemTime::now()))
        .header("Content-Type", "application/json")
        .body(String::from(r#"{"message": "404: Not Found", "code": 0}"#))
        .unwrap())
}

pub fn updates_stable(p: HashMap<String, String>) -> warp::http::Result<Response<String>> {
    let builder = Response::builder().header("Date", httpdate::fmt_http_date(SystemTime::now()));

    let version = p
        .get("version")
        .and_then(|v| v.parse::<Version>().ok())
        .unwrap_or_default();

    let res = if let Ok(platform) = p
        .get("platform")
        .map(|p| p.clone())
        .unwrap_or(String::from("osx"))
        .parse::<Platform>()
    {
        let (latest_version, pub_date) = VERSION_MAP[platform].clone();
        if version < latest_version {
            let extra_json = if platform == Platform::OsX {
                format!(
                    r#", "url": "https://dl.discordapp.net/apps/osx/{latest_version}/Discord.zip", "notes": """#
                )
            } else {
                String::new()
            };
            builder
                .header("Content-Type", "application/json")
                .body(format!(
                    r#"{{"name": "{latest_version}", "pub_date": "{pub_date}"{extra_json}}}\n"#
                ))
        } else {
            builder
                .status(204)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(String::new())
        }
    } else {
        builder
            .status(404)
            .header("Content-Type", "application/json")
            .body(String::from(r#"{"message": "404: Not Found", "code": 0}"#))
    };

    println!("{:?}", res);
    res
}
