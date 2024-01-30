// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::events::models::{ClientInfo, Status};
use serde::{Deserialize, Serialize};
use twilight_model::gateway::presence::Activity;
use twilight_model::id::marker::GenericMarker;
use twilight_model::id::Id;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Session {
    pub activities: Vec<Activity>,
    pub client_info: ClientInfo,
    pub sesion_id: Id<GenericMarker>,
    pub status: Status,
}
