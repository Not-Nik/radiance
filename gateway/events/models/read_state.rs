// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use twilight_model::id::marker::MessageMarker;
use twilight_model::id::Id;
use twilight_model::util::Timestamp;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadState {
    pub entries: Vec<ReadStateEntry>,
    pub partial: bool,
    pub version: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadStateEntry {
    pub flags: u32,
    pub id: String,
    pub last_message_id: Id<MessageMarker>,
    pub last_pin_timestamp: Timestamp,
    pub mention_count: u32,
}
