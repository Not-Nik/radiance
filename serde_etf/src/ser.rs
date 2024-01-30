use std::collections::HashMap;
use eetf::{Atom, BigInteger, Binary, FixInteger, Float, List, Map, Term, Tuple};
use serde::{ser, Serialize};

use crate::error::{Error, Result};

pub struct Serializer {}

pub struct SeqSerializer {
    elements: Vec<Term>,
}

pub struct MapSerializer {
    map: HashMap<Term, Term>
}

pub fn to_term<T>(value: &T) -> Result<Term>
where
    T: Serialize,
{
    let mut serializer = Serializer {};
    value.serialize(&mut serializer)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = Term;

    type Error = Error;

    type SerializeSeq = SeqSerializer;
    type SerializeTuple = SeqSerializer;
    type SerializeTupleStruct = SeqSerializer;
    type SerializeTupleVariant = Self;
    type SerializeMap = MapSerializer;
    type SerializeStruct = MapSerializer;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Term> {
        Ok(Term::Atom(Atom::from(if v { "true" } else { "false" })))
    }

    fn serialize_i8(self, v: i8) -> Result<Term> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Term> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Term> {
        Ok(Term::FixInteger(FixInteger::from(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Term> {
        Ok(Term::BigInteger(BigInteger::from(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Term> {
        self.serialize_i16(i16::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Term> {
        Ok(Term::FixInteger(FixInteger::from(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Term> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Term> {
        Ok(Term::BigInteger(BigInteger::from(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Term> {
        // Float::try_from(f32) can only fail if we try to encode +inf/-inf or NaN
        Ok(Term::Float(
            Float::try_from(v).map_err(|_| Error::NonFiniteFloat)?,
        ))
    }

    fn serialize_f64(self, v: f64) -> Result<Term> {
        // Float::try_from(f64) can only fail if we try to encode +inf/-inf or NaN
        Ok(Term::Float(
            Float::try_from(v).map_err(|_| Error::NonFiniteFloat)?,
        ))
    }

    fn serialize_char(self, v: char) -> Result<Term> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Term> {
        Ok(Term::Binary(Binary::from(v.as_bytes())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Term> {
        Ok(Term::Binary(Binary::from(v)))
    }

    fn serialize_none(self) -> Result<Term> {
        Ok(Term::Atom(Atom::from("nil")))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Term>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Term> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Term> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Term> {
        self.serialize_u32(variant_index)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Term>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Term>
    where
        T: ?Sized + Serialize,
    {
        Ok(Term::Tuple(Tuple::from(vec![
            self.serialize_u32(variant_index)?,
            value.serialize(self)?,
        ])))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer {
            elements: len.map(|l| Vec::with_capacity(l)).unwrap_or_default(),
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(SeqSerializer {
            elements: Vec::with_capacity(len),
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer {
            map: len.map(|l| HashMap::with_capacity(l)).unwrap_or_default(),
        })
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl ser::SerializeSeq for SeqSerializer {
    type Ok = Term;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.elements.push(value.serialize(&mut Serializer {})?);
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<Term> {
        Ok(Term::List(List::from(self.elements)))
    }
}

impl ser::SerializeTuple for SeqSerializer {
    type Ok = Term;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.elements.push(value.serialize(&mut Serializer {})?);
        Ok(())
    }

    fn end(self) -> Result<Term> {
        Ok(Term::Tuple(Tuple::from(self.elements)))
    }
}

impl ser::SerializeTupleStruct for SeqSerializer {
    type Ok = Term;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.elements.push(value.serialize(&mut Serializer {})?);
        Ok(())
    }

    fn end(self) -> Result<Term> {
        Ok(Term::Tuple(Tuple::from(self.elements)))
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = Term;
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Term> {
        todo!()
    }
}

impl ser::SerializeMap for MapSerializer {
    type Ok = Term;
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::SerializeKey)
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::SerializeValue)
    }

    fn serialize_entry<K: ?Sized, V: ?Sized>(
        &mut self,
        key: &K,
        value: &V,
    ) -> std::result::Result<(), Self::Error>
    where
        K: Serialize,
        V: Serialize,
    {
        let mut serializer = Serializer{};
        self.map.insert(key.serialize(&mut serializer)?, value.serialize(&mut serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Term> {
        Ok(Term::Map(Map::from(self.map)))
    }
}

impl ser::SerializeStruct for MapSerializer {
    type Ok = Term;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer{};
        self.map.insert(key.serialize(&mut serializer)?, value.serialize(&mut serializer)?);
        Ok(())
    }

    fn end(self) -> Result<Term> {
        Ok(Term::Map(Map::from(self.map)))
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = Term;
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Term> {
        todo!()
    }
}

////////////////////////////////////////////////////////////////////////////////
