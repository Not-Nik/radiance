// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use twilight_model::id::marker::GuildMarker;
use twilight_model::id::Id;
use twilight_model::voice::VoiceState;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuildSupplemental {
    pub embed_activities: Vec<()>,
    pub id: Id<GuildMarker>,
    pub voice_states: Vec<VoiceState>,
}
