// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::events::models::{ClientInfo, Status};
use crate::events::{EventPayload, IntoPayload, RadianceEvent};
use serde::{Deserialize, Serialize};
use twilight_model::gateway::presence::Activity;
use twilight_model::gateway::OpCode;
use twilight_model::id::marker::GenericMarker;
use twilight_model::id::Id;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Session {
    pub activities: Vec<Activity>,
    pub client_info: ClientInfo,
    pub sesion_id: Id<GenericMarker>,
    pub status: Status,
}

impl IntoPayload for Vec<Session> {
    fn into_payload(self, s: &mut u32) -> EventPayload {
        *s += 1;

        EventPayload {
            op: OpCode::Dispatch,
            d: Some(RadianceEvent::SessionsReplace(self)),
            s: Some(*s),
            t: Some("SESSIONS_REPLACE".to_string()),
        }
    }
}
