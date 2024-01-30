// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod identify;
pub mod models;
mod ready;
mod ready_supplemental;

pub use identify::*;
pub use ready::*;
pub use ready_supplemental::*;

use models::Session;
use serde::ser::SerializeStruct;
use serde::Serialize;
use serde_json::{Map, Value};
use twilight_model::gateway::event::Event;
use twilight_model::gateway::OpCode;

#[derive(Clone, Debug, PartialEq)]
pub enum RadianceEvent {
    Identify(Identify),
    Ready(Ready),
    ReadySupplemental(ReadySupplemental),
    SessionsReplace(Vec<Session>),
    Twilight(Event),
}

// Serialize is implemented below, because Event doesn't directly implement it
#[derive(Debug)]
pub struct EventPayload {
    op: OpCode,
    d: Option<RadianceEvent>,
    s: Option<u32>,
    t: Option<String>,
}

impl EventPayload {
    pub fn heartbeat_ack() -> EventPayload {
        EventPayload {
            op: OpCode::HeartbeatAck,
            d: None,
            s: None,
            t: None,
        }
    }
}

pub trait IntoPayload {
    fn into_payload(self, s: &mut u32) -> EventPayload;
}

impl Serialize for EventPayload {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("EventPayload", 4)?;
        state.serialize_field("op", &self.op)?;

        if let Some(RadianceEvent::Twilight(tw)) = &self.d {
            // Manually serialize Event based on its variants
            match tw {
                Event::GatewayClose(_) => state.serialize_field("d", &Value::Object(Map::new())),
                Event::GatewayHeartbeatAck => {
                    state.serialize_field("d", &Value::Object(Map::new()))
                }
                Event::GatewayReconnect => state.serialize_field("d", &Value::Object(Map::new())),
                Event::GiftCodeUpdate => state.serialize_field("d", &Value::Object(Map::new())),
                Event::PresencesReplace => state.serialize_field("d", &Value::Object(Map::new())),
                Event::Resumed => state.serialize_field("d", &Value::Object(Map::new())),

                Event::AutoModerationActionExecution(x) => state.serialize_field("d", x),
                Event::AutoModerationRuleCreate(x) => state.serialize_field("d", x),
                Event::AutoModerationRuleDelete(x) => state.serialize_field("d", x),
                Event::AutoModerationRuleUpdate(x) => state.serialize_field("d", x),
                Event::BanAdd(x) => state.serialize_field("d", x),
                Event::BanRemove(x) => state.serialize_field("d", x),
                Event::ChannelCreate(x) => state.serialize_field("d", x),
                Event::ChannelDelete(x) => state.serialize_field("d", x),
                Event::ChannelPinsUpdate(x) => state.serialize_field("d", x),
                Event::ChannelUpdate(x) => state.serialize_field("d", x),
                Event::CommandPermissionsUpdate(x) => state.serialize_field("d", x),
                Event::GatewayHeartbeat(x) => state.serialize_field("d", x),
                Event::GatewayHello(x) => state.serialize_field("d", x),
                Event::GatewayInvalidateSession(x) => state.serialize_field("d", x),
                Event::GuildAuditLogEntryCreate(x) => state.serialize_field("d", x),
                Event::GuildCreate(x) => state.serialize_field("d", x),
                Event::GuildDelete(x) => state.serialize_field("d", x),
                Event::GuildEmojisUpdate(x) => state.serialize_field("d", x),
                Event::GuildIntegrationsUpdate(x) => state.serialize_field("d", x),
                Event::GuildScheduledEventCreate(x) => state.serialize_field("d", x),
                Event::GuildScheduledEventDelete(x) => state.serialize_field("d", x),
                Event::GuildScheduledEventUpdate(x) => state.serialize_field("d", x),
                Event::GuildScheduledEventUserAdd(x) => state.serialize_field("d", x),
                Event::GuildScheduledEventUserRemove(x) => state.serialize_field("d", x),
                Event::GuildStickersUpdate(x) => state.serialize_field("d", x),
                Event::GuildUpdate(x) => state.serialize_field("d", x),
                Event::IntegrationCreate(x) => state.serialize_field("d", x),
                Event::IntegrationDelete(x) => state.serialize_field("d", x),
                Event::IntegrationUpdate(x) => state.serialize_field("d", x),
                Event::InteractionCreate(x) => state.serialize_field("d", x),
                Event::InviteCreate(x) => state.serialize_field("d", x),
                Event::InviteDelete(x) => state.serialize_field("d", x),
                Event::MemberAdd(x) => state.serialize_field("d", x),
                Event::MemberRemove(x) => state.serialize_field("d", x),
                Event::MemberUpdate(x) => state.serialize_field("d", x),
                Event::MemberChunk(x) => state.serialize_field("d", x),
                Event::MessageCreate(x) => state.serialize_field("d", x),
                Event::MessageDelete(x) => state.serialize_field("d", x),
                Event::MessageDeleteBulk(x) => state.serialize_field("d", x),
                Event::MessageUpdate(x) => state.serialize_field("d", x),
                Event::PresenceUpdate(x) => state.serialize_field("d", x),
                Event::ReactionAdd(x) => state.serialize_field("d", x),
                Event::ReactionRemove(x) => state.serialize_field("d", x),
                Event::ReactionRemoveAll(x) => state.serialize_field("d", x),
                Event::ReactionRemoveEmoji(x) => state.serialize_field("d", x),
                Event::Ready(x) => state.serialize_field("d", x),
                Event::RoleCreate(x) => state.serialize_field("d", x),
                Event::RoleDelete(x) => state.serialize_field("d", x),
                Event::RoleUpdate(x) => state.serialize_field("d", x),
                Event::StageInstanceCreate(x) => state.serialize_field("d", x),
                Event::StageInstanceDelete(x) => state.serialize_field("d", x),
                Event::StageInstanceUpdate(x) => state.serialize_field("d", x),
                Event::ThreadCreate(x) => state.serialize_field("d", x),
                Event::ThreadDelete(x) => state.serialize_field("d", x),
                Event::ThreadListSync(x) => state.serialize_field("d", x),
                Event::ThreadMemberUpdate(x) => state.serialize_field("d", x),
                Event::ThreadMembersUpdate(x) => state.serialize_field("d", x),
                Event::ThreadUpdate(x) => state.serialize_field("d", x),
                Event::TypingStart(x) => state.serialize_field("d", x),
                Event::UnavailableGuild(x) => state.serialize_field("d", x),
                Event::UserUpdate(x) => state.serialize_field("d", x),
                Event::VoiceServerUpdate(x) => state.serialize_field("d", x),
                Event::VoiceStateUpdate(x) => state.serialize_field("d", x),
                Event::WebhooksUpdate(x) => state.serialize_field("d", x),
            }?;
        } else {
            match &self.d {
                Some(RadianceEvent::Ready(r)) => state.serialize_field("d", r)?,
                Some(RadianceEvent::ReadySupplemental(r)) => state.serialize_field("d", r)?,
                Some(RadianceEvent::SessionsReplace(v)) => state.serialize_field("d", v)?,
                None => {}
                _ => unreachable!(),
            }
        }
        if self.s.is_some() {
            state.serialize_field("s", &self.s)?;
        }
        if self.t.is_some() {
            state.serialize_field("t", &self.t)?;
        }
        state.end()
    }
}

impl IntoPayload for Event {
    fn into_payload(self, s: &mut u32) -> EventPayload {
        let op = match &self {
            Event::GatewayHello(_) => OpCode::Hello,
            Event::GatewayHeartbeatAck => OpCode::HeartbeatAck,
            Event::GatewayInvalidateSession(_) => OpCode::InvalidSession,
            _ => OpCode::Dispatch,
        };

        let event_name = self.kind().name().map(|s| s.to_string());

        EventPayload {
            op,
            d: Some(RadianceEvent::Twilight(self)),
            s: if op == OpCode::Dispatch {
                *s += 1;
                Some(*s)
            } else {
                None
            },
            t: event_name,
        }
    }
}
