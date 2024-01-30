// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::events::models::{
    Authentication, Consent, Friend, Guild, NotificationSettings, PrivateChannel, ReadState,
    Relationship, Session, User, UserGuildSettings,
};
use crate::events::{EventPayload, IntoPayload, RadianceEvent};
use serde::{Deserialize, Serialize};
use twilight_model::gateway::OpCode;
use twilight_model::guild::Member;
use twilight_model::id::marker::GenericMarker;
use twilight_model::id::Id;
use twilight_model::user::Connection;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Ready {
    pub analytics_token: String,
    pub api_code_version: u32,
    pub auth: Authentication,
    pub auth_session_id_hash: String,
    pub connected_accounts: Vec<Connection>,
    pub consents: Consent,
    pub country_code: String,
    pub current_location: Vec<String>,
    pub experiments: Vec<Vec<[i32; 8]>>,
    pub friend_suggestion_count: u32,
    pub geo_ordered_rtc_regions: Vec<String>,
    pub guild_experiments: Vec<Vec<u32>>,
    pub guild_join_requests: Vec<String>,
    pub guilds: Vec<Guild>,
    pub merged_members: Vec<Vec<Member>>,
    pub notification_settings: NotificationSettings,
    pub private_channels: Vec<PrivateChannel>,
    pub read_state: ReadState,
    pub relationships: Vec<Relationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_action: Option<String>,
    pub resume_gateway_url: String,
    pub session_id: Id<GenericMarker>,
    pub session_type: String, // possible values: "normal",
    pub sessions: Vec<Session>,
    pub tutorial: Option<()>,
    pub user: User,
    pub user_guild_settings: UserGuildSettings,
    pub user_settings_proto: String,
    pub users: Vec<Friend>,
    pub v: u32,
}

impl IntoPayload for Ready {
    fn into_payload(self, s: &mut u32) -> EventPayload {
        *s += 1;

        EventPayload {
            op: OpCode::Dispatch,
            d: Some(RadianceEvent::Ready(self)),
            s: Some(*s),
            t: Some("READY".to_string()),
        }
    }
}
