// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use twilight_model::id::marker::GenericMarker;
use twilight_model::id::Id;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Resume {
    pub token: String,
    pub session_id: Id<GenericMarker>,
    pub seq: u32,
}
