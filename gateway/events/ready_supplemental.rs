// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::events::models::{GuildSupplemental, MergedPresences, PrivateChannel};
use crate::events::{EventPayload, IntoPayload, RadianceEvent};
use serde::{Deserialize, Serialize};
use twilight_model::gateway::OpCode;
use twilight_model::guild::Member;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadySupplemental {
    pub disclose: Vec<String>, // can include "pomelo"
    pub game_invites: Vec<()>,
    pub guilds: Vec<GuildSupplemental>,
    pub lazy_private_channels: Vec<PrivateChannel>, // not sure about the type
    pub merged_members: Vec<Vec<Member>>,
    pub merged_presences: MergedPresences,
}

impl IntoPayload for ReadySupplemental {
    fn into_payload(self, s: &mut u32) -> EventPayload {
        *s += 1;

        EventPayload {
            op: OpCode::Dispatch,
            d: Some(RadianceEvent::ReadySupplemental(self)),
            s: Some(*s),
            t: Some("READY_SUPPLEMENTAL".to_string()),
        }
    }
}
