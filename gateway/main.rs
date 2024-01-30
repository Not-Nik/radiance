// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![feature(async_closure)]

mod connection;
mod encoding;
mod error;
mod event_deserializer;
mod events;

use crate::connection::GatewayConnection;
use crate::encoding::Encoding;
use crate::error::GatewayError;
use crate::events::models::{
    Authentication, ClientInfo, Consent, MergedPresences, NotificationSettings, Personalization,
    ReadState, Session, Status, User, UserGuildSettings,
};
use crate::events::{EventPayload, IntoPayload, RadianceEvent, Ready, ReadySupplemental};
use log::debug;
use std::collections::HashMap;
use std::str::FromStr;
use twilight_model::gateway::event::Event;
use twilight_model::gateway::payload::incoming::Hello;
use twilight_model::id::Id;
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

    let identify = match connection.read_event().await? {
        RadianceEvent::Identify(identify) => identify,
        RadianceEvent::Resume(_) => {
            let invalid_session = Event::GatewayInvalidateSession(false);

            connection
                .send_event(invalid_session.into_payload(&mut sequence))
                .await?;
            return Ok(());
        }
        _ => {
            return Err(GatewayError::UnexpectedPayload);
        }
    };

    debug!("Received identify");

    let session = Session {
        activities: vec![],
        client_info: ClientInfo {
            client: "web".to_string(),
            os: identify.properties.os,
            version: 0,
        },
        sesion_id: Id::new(1),
        status: Status::Online,
    };

    let ready = Ready {
        analytics_token: "OTY3NDc4MTA3NTUwMzg4MjM1.gBFnffeJgsNj6npFfnYxcabuolk".to_string(),
        api_code_version: 1,
        auth: Authentication {
            authenticator_types: vec![],
        },
        auth_session_id_hash: "cJ9xGK8GACC1G+HofFNJHb6VdVx95Uy1/ycpFy5Y9Sc=".to_string(),
        connected_accounts: vec![],
        consents: Consent {
            personalization: Personalization { consented: false },
        },
        country_code: "DE".to_string(),
        current_location: vec!["DE".to_string(), "DE:BE".to_string()],
        experiments: vec![],
        friend_suggestion_count: 0,
        geo_ordered_rtc_regions: vec![
            "frankfurt".to_string(),
            "frankfurt-two".to_string(),
            "rotterdam".to_string(),
            "stockholm".to_string(),
            "milan".to_string(),
        ],
        guild_experiments: vec![],
        guild_join_requests: vec![],
        guilds: vec![],
        merged_members: vec![],
        notification_settings: NotificationSettings { flags: 0 },
        private_channels: vec![],
        read_state: ReadState {
            entries: vec![],
            partial: false,
            version: 0,
        },
        relationships: vec![],
        required_action: None,
        resume_gateway_url: "wss://gateway.discord.gg".to_string(),
        session_id: Id::new(1),
        session_type: "normal".to_string(),
        sessions: vec![session.clone()],
        tutorial: None,
        user: User {
            accent_color: None,
            avatar: None,
            avatar_decoration_data: None,
            banner: None,
            banner_color: None,
            bio: "".to_string(),
            desktop: false,
            discriminator: 0,
            email: "mail@example.com".to_string(),
            flags: 0,
            global_name: None,
            id: Id::new(1),
            mfa_enabled: false,
            mobile: false,
            nsfw_allowed: false,
            phone: None,
            premium: false,
            premium_type: 0,
            pronouns: "".to_string(),
            purchased_flags: 0,
            username: "Person".to_string(),
            verified: true,
        },
        user_guild_settings: UserGuildSettings {
            entries: vec![],
            partial: false,
            version: 0,
        },
        user_settings_proto: "".to_string(),
        users: vec![],
        v: 9,
    };

    connection
        .send_event(ready.into_payload(&mut sequence))
        .await?;

    debug!("Sent ready");

    let ready_supplemental = ReadySupplemental {
        disclose: vec!["pomelo".to_string()],
        game_invites: vec![],
        guilds: vec![],
        lazy_private_channels: vec![],
        merged_members: vec![],
        merged_presences: MergedPresences {
            friends: vec![],
            guilds: vec![],
        },
    };

    connection
        .send_event(ready_supplemental.into_payload(&mut sequence))
        .await?;

    debug!("Sent ready supplemental");

    let sessions_replace = vec![session.clone()];

    connection
        .send_event(sessions_replace.into_payload(&mut sequence))
        .await?;

    debug!("Sent sessions replace");

    loop {
        let event = connection.read_event().await?;

        match event {
            RadianceEvent::Twilight(Event::GatewayHeartbeat(_)) => {
                connection.send_event(EventPayload::heartbeat_ack()).await?;
                debug!("Heartbeat");
            }
            _ => {
                debug!("{:?}", event);
            }
        };
    }
}
