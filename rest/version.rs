// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::version::Platform::{Linux, OsX, Win};
use enum_map::Enum;
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

#[derive(Copy, Clone, Debug, Enum, Eq, Hash, PartialEq)]
pub enum Platform {
    OsX,
    Win,
    Linux,
}

pub enum VersionParseError {
    InvalidFormat,
    InvalidPart,
}

impl Version {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::new(0, 0, 0)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        } else if self.lt(other) {
            return Some(Ordering::Less);
        } else {
            return Some(Ordering::Greater);
        }
    }

    fn lt(&self, other: &Self) -> bool {
        if self.major < other.major {
            return true;
        } else if self.major > other.major {
            return false;
        }
        if self.minor < other.minor {
            return true;
        } else if self.minor > other.minor {
            return false;
        }
        if self.patch < other.patch {
            return true;
        }
        return false;
    }

    fn le(&self, other: &Self) -> bool {
        if self.major < other.major {
            return true;
        } else if self.major > other.major {
            return false;
        }
        if self.minor < other.minor {
            return true;
        } else if self.minor > other.minor {
            return false;
        }
        if self.patch < other.patch {
            return true;
        }
        return true;
    }

    fn gt(&self, other: &Self) -> bool {
        if self.major > other.major {
            return true;
        } else if self.major < other.major {
            return false;
        }
        if self.minor > other.minor {
            return true;
        } else if self.minor < other.minor {
            return false;
        }
        if self.patch > other.patch {
            return true;
        }
        return false;
    }

    fn ge(&self, other: &Self) -> bool {
        if self.major > other.major {
            return true;
        } else if self.major < other.major {
            return false;
        }
        if self.minor > other.minor {
            return true;
        } else if self.minor < other.minor {
            return false;
        }
        if self.patch > other.patch {
            return true;
        }
        return true;
    }
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('.');
        let major_string = split.next().ok_or(VersionParseError::InvalidFormat)?;
        let minor_string = split.next().ok_or(VersionParseError::InvalidFormat)?;
        let patch_string = split.next().ok_or(VersionParseError::InvalidFormat)?;

        if split.next().is_some() {
            return Err(VersionParseError::InvalidFormat);
        }

        Ok(Version {
            major: major_string
                .parse()
                .map_err(|_| VersionParseError::InvalidPart)?,
            minor: minor_string
                .parse()
                .map_err(|_| VersionParseError::InvalidPart)?,
            patch: patch_string
                .parse()
                .map_err(|_| VersionParseError::InvalidPart)?,
        })
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Version({}.{}.{})", self.major, self.minor, self.patch)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "osx" => Ok(OsX),
            "win" => Ok(Win),
            "linux" => Ok(Linux),
            _ => Err(()),
        }
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OsX => write!(f, "osx"),
            Win => write!(f, "win"),
            Linux => write!(f, "linux"),
        }
    }
}
