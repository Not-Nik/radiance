// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use twilight_model::channel::message::Sticker;
use twilight_model::channel::{Channel, StageInstance};
use twilight_model::guild;
use twilight_model::guild::scheduled_event::GuildScheduledEvent;
use twilight_model::guild::{Emoji, Role};
use twilight_model::id::marker::GuildMarker;
use twilight_model::id::Id;
use twilight_model::util::Timestamp;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Guild {
    pub application_command_counts: HashMap<String, String>,
    pub channels: Vec<Channel>,
    pub data_mode: String, // possible values: "full",
    pub emojis: Vec<Emoji>,
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,
    pub id: Id<GuildMarker>,
    pub joined_at: Timestamp,
    pub large: bool,
    pub member_count: u32,
    pub premium_subscription_count: u32,
    pub properties: guild::Guild, // shouldn't have roles, channels, stickers or emojis
    pub roles: Vec<Role>,
    pub stage_instances: Vec<StageInstance>,
    pub stickers: Vec<Sticker>,
    pub threads: Vec<Channel>,
    pub version: u32, // something like 1706402981462
}
