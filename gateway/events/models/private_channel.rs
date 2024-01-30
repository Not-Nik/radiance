// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};
use twilight_model::channel::{ChannelFlags, ChannelType};
use twilight_model::id::marker::{ChannelMarker, MessageMarker, UserMarker};
use twilight_model::id::Id;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PrivateChannel {
    pub flags: ChannelFlags,
    pub id: Id<ChannelMarker>,
    pub is_spam: bool,
    pub last_message_id: Id<MessageMarker>,
    pub recipient_ids: Vec<Id<UserMarker>>,
    pub safety_warnings: Vec<()>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
}
