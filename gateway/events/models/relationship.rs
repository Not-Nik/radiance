// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;
use twilight_model::util::Timestamp;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Relationship {
    pub id: Id<UserMarker>,
    pub nickname: Option<String>,
    pub since: Timestamp,
    #[serde(rename = "type")]
    pub kind: u32, // 3 for friends
    pub user_id: Id<UserMarker>,
}
