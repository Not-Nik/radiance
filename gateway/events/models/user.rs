// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;
use twilight_model::util::ImageHash;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct User {
    pub accent_color: Option<u32>,
    pub avatar: Option<ImageHash>,
    pub avatar_decoration_data: Option<ImageHash>,
    pub banner: Option<ImageHash>,
    pub banner_color: Option<u32>,
    pub bio: String,
    pub desktop: bool,
    pub discriminator: u16,
    pub email: String,
    pub flags: i64,
    pub global_name: Option<String>,
    pub id: Id<UserMarker>,
    pub mfa_enabled: bool,
    pub mobile: bool,
    pub nsfw_allowed: bool,
    pub phone: Option<String>,
    pub premium: bool,
    pub premium_type: i64,
    pub pronouns: String,
    pub purchased_flags: i64,
    pub username: String,
    pub verified: bool,
}
