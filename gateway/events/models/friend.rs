// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Friend {
    pub avatar: String,
    pub avatar_decoration_data: Option<()>,
    pub bot: bool,
    pub discriminator: String,
    pub display_name: String,
    pub global_name: String,
    pub id: Id<UserMarker>,
    pub public_flags: u32,
    pub username: String,
}
