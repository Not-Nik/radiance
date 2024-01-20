use crate::version::{Platform, Version};
use crate::VERSION_MAP;
use std::collections::HashMap;
use std::time::SystemTime;
use warp::http::Response;

pub fn updates_stable(p: HashMap<String, String>) -> warp::http::Result<Response<String>> {
    let builder = Response::builder()
        .header("Date", httpdate::fmt_http_date(SystemTime::now()));

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
