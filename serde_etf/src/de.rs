use eetf::{BigInteger, Term};
use std::collections::HashMap;

use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, VariantAccess, Visitor};
use serde::{forward_to_deserialize_any, Deserialize};

use crate::error::{Error, Result};

pub struct Deserializer {
    input: Term,
}

impl Deserializer {
    pub fn from_term(input: Term) -> Self {
        Deserializer { input }
    }
}

pub fn from_term<T>(t: Term) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let deserializer = Deserializer::from_term(t);
    T::deserialize(deserializer)
}

impl Deserializer {
    fn deserialize_integer<'de, V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::FixInteger(i) => visitor.visit_i32(i.value),
            Term::BigInteger(i) => Self::deserialize_bigint(i, visitor),
            _ => Err(Error::ExpectedInt),
        }
    }

    fn deserialize_bigint<'de, V>(i: BigInteger, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let Ok(num) = i.to_string().parse::<i64>() {
            visitor.visit_i64(num)
        } else if !i.to_string().starts_with('-') {
            if let Ok(num) = i.to_string().parse::<u64>() {
                visitor.visit_u64(num)
            } else {
                Err(Error::NumberTooLarge)
            }
        } else {
            // Number is negative, but doesn't fit into i64
            Err(Error::NumberTooSmall)
        }
    }
}

impl<'de> de::Deserializer<'de> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::Atom(a) => visitor.visit_string(a.name),
            Term::FixInteger(i) => visitor.visit_i32(i.value),
            Term::BigInteger(i) => Self::deserialize_bigint(i, visitor),
            Term::Float(f) => visitor.visit_f64(f.value),
            Term::Pid(_) => Err(Error::InvalidInput),
            Term::Port(_) => Err(Error::InvalidInput),
            Term::Reference(_) => Err(Error::InvalidInput),
            Term::ExternalFun(_) => Err(Error::InvalidInput),
            Term::InternalFun(_) => Err(Error::InvalidInput),
            Term::Binary(b) => visitor.visit_byte_buf(b.bytes),
            Term::BitBinary(_) => Err(Error::InvalidInput),
            Term::ByteList(_) => Err(Error::InvalidInput),
            Term::List(l) => visitor.visit_seq(SeqDeserializer::new(l.elements)),
            Term::ImproperList(_) => Err(Error::InvalidInput),
            Term::Tuple(t) => visitor.visit_seq(SeqDeserializer::new(t.elements)),
            Term::Map(m) => visitor.visit_map(MapDeserializer::new(m.map)),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::Atom(a) => match &*a.name {
                "false" => visitor.visit_bool(false),
                "true" => visitor.visit_bool(true),
                _ => Err(Error::ExpectedBool),
            },
            _ => Err(Error::ExpectedBool),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_f64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::Float(v) => visitor.visit_f64(v.value),
            _ => Err(Error::ExpectedFloat),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::Binary(v) => visitor.visit_byte_buf(v.bytes),
            _ => Err(Error::ExpectedString),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::Binary(b) => visitor.visit_byte_buf(b.bytes),
            _ => Err(Error::ExpectedBytes),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::Atom(ref a) => {
                if a.name == "nil" {
                    visitor.visit_none()
                } else {
                    Err(Error::InvalidInput)
                }
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::List(l) => visitor.visit_seq(SeqDeserializer::new(l.elements)),
            _ => Err(Error::ExpectedList),
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::Tuple(t) => {
                if t.elements.len() != len {
                    return Err(Error::WrongTupleLength);
                }
                visitor.visit_seq(SeqDeserializer::new(t.elements))
            }
            _ => Err(Error::ExpectedTuple),
        }
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            Term::Map(m) => visitor.visit_map(MapDeserializer::new(m.map)),
            _ => Err(Error::ExpectedMap),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(EnumDeserializer::new(self.input))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqDeserializer {
    iter: <Vec<Term> as IntoIterator>::IntoIter,
}

impl SeqDeserializer {
    fn new(vec: Vec<Term>) -> Self {
        SeqDeserializer {
            iter: vec.into_iter(),
        }
    }
}

impl<'de> de::Deserializer<'de> for SeqDeserializer {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let len = self.iter.len();
        if len == 0 {
            visitor.visit_unit()
        } else {
            let ret = visitor.visit_seq(&mut self)?;
            if self.iter.len() == 0 {
                Ok(ret)
            } else {
                Err(Error::ExtraneousInput)
            }
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(Deserializer::from_term(value)).map(Some),
            None => Ok(None),
        }
    }
}

struct MapDeserializer {
    iter: <HashMap<Term, Term> as IntoIterator>::IntoIter,
    value: Option<Term>,
}

impl MapDeserializer {
    fn new(map: HashMap<Term, Term>) -> Self {
        MapDeserializer {
            iter: map.into_iter(),
            value: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer {
    type Error = Error;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(Deserializer::from_term(key)).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(Deserializer::from_term(value)),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}

impl<'de> de::Deserializer<'de> for MapDeserializer {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

pub struct EnumDeserializer {
    variant: Term,
}

impl EnumDeserializer {
    pub fn new(variant: Term) -> EnumDeserializer {
        EnumDeserializer { variant }
    }
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = VariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let visitor = VariantDeserializer {};
        seed.deserialize(Deserializer::from_term(self.variant))
            .map(|v| (v, visitor))
    }
}

pub struct VariantDeserializer {}

impl<'de> VariantAccess<'de> for VariantDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        Err(Error::ExpectedTuple)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::ExpectedTuple)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::ExpectedMap)
    }
}
