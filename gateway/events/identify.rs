// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::events::models::{ClientProperties, ClientState, Presence};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Identify {
    pub token: String,
    pub capabilities: u32,
    pub properties: ClientProperties,
    pub presence: Presence,
    pub compress: bool,
    pub client_state: ClientState,
}
