// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ReleaseChannel {
    #[serde(rename = "stable")]
    Stable,
    // probably canary and ptb, but im unsure
}
