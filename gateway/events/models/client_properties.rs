// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::events::models::ReleaseChannel;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ClientProperties {
    pub os: String,
    pub browser: String,
    pub device: Option<String>,
    pub system_locale: String,
    pub browser_user_agent: String,
    pub browser_version: String,
    pub os_version: String,
    pub referrer: Option<String>,
    pub referring_domain: Option<String>,
    pub referrer_current: Option<String>,
    pub referring_domain_current: Option<String>,
    pub release_channel: ReleaseChannel,
    pub client_build_number: u32,
    pub client_event_source: Option<()>,
}
