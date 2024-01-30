// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use twilight_model::id::marker::{ChannelMarker, GuildMarker};
use twilight_model::id::Id;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VoiceStateUpdate {
    pub guild_id: Option<Id<GuildMarker>>,
    pub channel_id: Option<Id<ChannelMarker>>,
    pub self_mute: bool,
    pub self_deaf: bool,
    pub self_video: bool,
    pub flags: u32,
}
