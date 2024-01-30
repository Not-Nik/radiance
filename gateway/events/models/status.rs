// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Status {
    #[serde(rename = "dnd")]
    DoNotDisturb,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "invisible")]
    Invisible,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "unknown")]
    Unknown,
}
