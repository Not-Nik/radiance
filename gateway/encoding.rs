// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Encoding {
    Etf,
    Json,
}

impl FromStr for Encoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "etf" => Ok(Encoding::Etf),
            "json" => Ok(Encoding::Json),
            _ => Err(()),
        }
    }
}

impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Encoding::Etf => f.write_str("etf"),
            Encoding::Json => f.write_str("json"),
        }
    }
}
