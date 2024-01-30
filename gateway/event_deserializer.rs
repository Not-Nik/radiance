// radiance (c) Nikolas Wipper 2024

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::events::{Identify, RadianceEvent, VoiceStateUpdate};
use serde::de::value::U8Deserializer;
use serde::de::{
    DeserializeSeed, Error, IgnoredAny, IntoDeserializer, MapAccess, Unexpected, Visitor,
};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::fmt::Formatter;
use std::str::FromStr;
use twilight_model::gateway::event::{DispatchEventWithTypeDeserializer, Event, GatewayEvent};
use twilight_model::gateway::payload::incoming::Hello;
use twilight_model::gateway::OpCode;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Field {
    D,
    Op,
    S,
    T,
}

pub struct EventDeserializer<'a> {
    event_type: Option<Cow<'a, str>>,
    op: u8,
    sequence: Option<u64>,
}

impl<'a> EventDeserializer<'a> {
    pub fn new(op: u8, event_type: Option<&'a str>) -> Self {
        Self {
            event_type: event_type.map(Into::into),
            op,
            sequence: None,
        }
    }

    /// Create a gateway event deserializer by scanning the JSON payload for its
    /// opcode and dispatch event type.
    pub fn from_json(input: &'a str) -> Option<Self> {
        let op = Self::find_opcode(input)?;
        let event_type = Self::find_event_type(input).map(Into::into);
        let sequence = Self::find_sequence(input);

        Some(Self {
            event_type,
            op,
            sequence,
        })
    }

    /// Create a deserializer with an owned event type.
    ///
    /// This is necessary when using a mutable deserialization library such as
    /// `simd-json`.
    pub fn into_owned(self) -> EventDeserializer<'static> {
        EventDeserializer {
            event_type: self
                .event_type
                .map(|event_type| Cow::Owned(event_type.into_owned())),
            op: self.op,
            sequence: self.sequence,
        }
    }

    /// Consume the deserializer, returning its components.
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_parts(self) -> (u8, Option<u64>, Option<Cow<'a, str>>) {
        (self.op, self.sequence, self.event_type)
    }

    /// Dispatch event type of the payload.
    pub fn event_type(&self) -> Option<&str> {
        self.event_type.as_deref()
    }

    /// Opcode of the payload.
    pub const fn op(&self) -> u8 {
        self.op
    }

    /// Sequence of the payload.
    ///
    /// May only be available if the deserializer was created via
    /// [`from_json`][`Self::from_json`]
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    fn find_event_type(input: &'a str) -> Option<&'a str> {
        // We're going to search for the event type key from the start. Discord
        // always puts it at the front before the D key from some testing of
        // several hundred payloads.
        //
        // If we find it, add 4, since that's the length of what we're searching
        // for.
        let from = input.find(r#""t":"#)? + 4;

        // Now let's find where the value starts, which may be a string or null.
        // Or maybe something else. If it's anything but a string, then there's
        // no event type.
        let start = input.get(from..)?.find(|c: char| !c.is_whitespace())? + from + 1;

        // Check if the character just before the cursor is '"'.
        if input.as_bytes().get(start - 1).copied()? != b'"' {
            return None;
        }

        let to = input.get(start..)?.find('"')?;

        input.get(start..start + to)
    }

    fn find_opcode(input: &'a str) -> Option<u8> {
        Self::find_integer(input, r#""op":"#)
    }

    fn find_sequence(input: &'a str) -> Option<u64> {
        Self::find_integer(input, r#""s":"#)
    }

    fn find_integer<T: FromStr>(input: &'a str, key: &str) -> Option<T> {
        // Find the op key's position and then search for where the first
        // character that's not base 10 is. This'll give us the bytes with the
        // op which can be parsed.
        //
        // Add 5 at the end since that's the length of what we're finding.
        let from = input.find(key)? + key.len();

        // Look for the first thing that isn't a base 10 digit or whitespace,
        // i.e. a comma (denoting another JSON field), curly brace (end of the
        // object), etc. This'll give us the op number, maybe with a little
        // whitespace.
        let to = input.get(from..)?.find(&[',', '}'] as &[_])?;
        // We might have some whitespace, so let's trim this.
        let clean = input.get(from..from + to)?.trim();

        T::from_str(clean).ok()
    }
}

struct EventVisitor<'a>(u8, Option<u64>, Option<Cow<'a, str>>);

impl EventVisitor<'_> {
    fn field<'de, T: Deserialize<'de>, V: MapAccess<'de>>(
        map: &mut V,
        field: Field,
    ) -> Result<T, V::Error> {
        let mut found = None;

        loop {
            match map.next_key::<Field>() {
                Ok(Some(key)) if key == field => found = Some(map.next_value()?),
                Ok(Some(_)) | Err(_) => {
                    map.next_value::<IgnoredAny>()?;

                    continue;
                }
                Ok(None) => {
                    break;
                }
            }
        }

        found.ok_or_else(|| {
            V::Error::missing_field(match field {
                Field::D => "d",
                Field::Op => "op",
                Field::S => "s",
                Field::T => "t",
            })
        })
    }

    fn ignore_all<'de, V: MapAccess<'de>>(map: &mut V) -> Result<(), V::Error> {
        while let Ok(Some(_)) | Err(_) = map.next_key::<Field>() {
            map.next_value::<IgnoredAny>()?;
        }

        Ok(())
    }
}

impl<'de> Visitor<'de> for EventVisitor<'_> {
    type Value = RadianceEvent;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("struct GatewayEvent")
    }

    #[allow(clippy::too_many_lines)]
    fn visit_map<V>(self, mut map: V) -> Result<RadianceEvent, V::Error>
    where
        V: MapAccess<'de>,
    {
        static VALID_OPCODES: &[&str] = &[
            "EVENT",
            "HEARTBEAT",
            "HEARTBEAT_ACK",
            "HELLO",
            "IDENTIFY",
            "INVALID_SESSION",
            "RECONNECT",
        ];

        let op_deser: U8Deserializer<V::Error> = self.0.into_deserializer();

        let op = OpCode::deserialize(op_deser).ok().ok_or_else(|| {
            let unexpected = Unexpected::Unsigned(u64::from(self.0));

            V::Error::invalid_value(unexpected, &"an opcode")
        })?;

        Ok(match op {
            OpCode::Dispatch => {
                let t = self
                    .2
                    .ok_or_else(|| V::Error::custom("event type not provided beforehand"))?;

                let mut d = None;
                let mut s = None;

                loop {
                    let key = match map.next_key() {
                        Ok(Some(key)) => key,
                        Ok(None) => break,
                        Err(_) => {
                            map.next_value::<IgnoredAny>()?;

                            continue;
                        }
                    };

                    match key {
                        Field::D => {
                            if d.is_some() {
                                return Err(V::Error::duplicate_field("d"));
                            }

                            let deserializer = DispatchEventWithTypeDeserializer::new(&t);

                            d = Some(map.next_value_seed(deserializer)?);
                        }
                        Field::S => {
                            if s.is_some() {
                                return Err(V::Error::duplicate_field("s"));
                            }

                            s = Some(map.next_value()?);
                        }
                        Field::Op | Field::T => {
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                }

                let d = d.ok_or_else(|| V::Error::missing_field("d"))?;
                let s = s.ok_or_else(|| V::Error::missing_field("s"))?;

                RadianceEvent::Twilight(Event::from(GatewayEvent::Dispatch(s, d)))
            }
            OpCode::Heartbeat => {
                let seq = Self::field(&mut map, Field::D)?;

                Self::ignore_all(&mut map)?;

                RadianceEvent::Twilight(Event::from(GatewayEvent::Heartbeat(seq)))
            }
            OpCode::HeartbeatAck => {
                Self::ignore_all(&mut map)?;

                RadianceEvent::Twilight(Event::from(GatewayEvent::HeartbeatAck))
            }
            OpCode::Hello => {
                let hello = Self::field::<Hello, _>(&mut map, Field::D)?;

                Self::ignore_all(&mut map)?;

                RadianceEvent::Twilight(Event::from(GatewayEvent::Hello(hello)))
            }
            OpCode::InvalidSession => {
                let invalidate = Self::field::<bool, _>(&mut map, Field::D)?;

                Self::ignore_all(&mut map)?;

                RadianceEvent::Twilight(Event::from(GatewayEvent::InvalidateSession(invalidate)))
            }
            OpCode::Identify => {
                let identify = Self::field::<Identify, _>(&mut map, Field::D)?;

                Self::ignore_all(&mut map)?;

                RadianceEvent::Identify(identify)
            }
            OpCode::Reconnect => {
                Self::ignore_all(&mut map)?;

                RadianceEvent::Twilight(Event::from(GatewayEvent::Reconnect))
            }
            OpCode::RequestGuildMembers => {
                return Err(V::Error::unknown_variant(
                    "RequestGuildMembers",
                    VALID_OPCODES,
                ))
            }
            OpCode::Resume => return Err(V::Error::unknown_variant("Resume", VALID_OPCODES)),
            OpCode::PresenceUpdate => {
                return Err(V::Error::unknown_variant("PresenceUpdate", VALID_OPCODES))
            }
            OpCode::VoiceStateUpdate => {
                let update = Self::field::<VoiceStateUpdate, _>(&mut map, Field::D)?;

                Self::ignore_all(&mut map)?;

                RadianceEvent::VoiceStateUpdate(update)
            }
            _ => {
                return Err(V::Error::unknown_variant(
                    "[non_exhaustive cover case]",
                    VALID_OPCODES,
                ))
            }
        })
    }
}

impl<'de> DeserializeSeed<'de> for EventDeserializer<'_> {
    type Value = RadianceEvent;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        const FIELDS: &[&str] = &["op", "d", "s", "t"];

        deserializer.deserialize_struct(
            "RadianceEvent",
            FIELDS,
            EventVisitor(self.op, self.sequence, self.event_type),
        )
    }
}
