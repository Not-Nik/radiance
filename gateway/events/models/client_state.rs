// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClientState {
    pub guild_versions: HashMap<(), ()>,
    pub highest_last_message_id: String, // this is an id, or zero, so we can't just put Id as the type
    pub read_state_version: u32,
    pub user_guild_settings_version: i32,
    pub private_channels_version: String,
    pub api_code_version: u32,
}
