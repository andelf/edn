use std::{str, collections::HashMap};

use crate::{
    error::{Error, Result},
    value::Value,
};

use serde::ser::{Impossible, Serialize};

use super::to_value;
use super::Key;

impl Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Value::Nil => serializer.serialize_unit(),
            Value::Boolean(b) => serializer.serialize_bool(*b),
            Value::Integer(n) => n.serialize(serializer),
            Value::Float(n) => n.serialize(serializer),
            Value::String(s) => serializer.serialize_str(s),
            Value::Vector(v) => v.serialize(serializer),
            Value::List(v) => v.serialize(serializer),
            Value::Set(v) => v.serialize(serializer),
            Value::Symbol(s) => s.serialize(serializer),
            Value::Keyword(s) => s.serialize(serializer),
            //Value::Instant(i) => i.serialize(serializer),
            //Value::Uuid(u) => u.serialize(serializer),
            Value::Character(c) => c.serialize(serializer),
            Value::Map(m) => {
                use serde::ser::SerializeMap;
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
            _ => unimplemented!(),
        }
    }
}

impl Serialize for Key {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        match self {
            Key::String(s) => serializer.serialize_str(s),
            Key::Symbol(s) => s.serialize(serializer),
            Key::Keyword(s) => s.serialize(serializer),
            Key::Character(c) => c.serialize(serializer),
            _ => unimplemented!(),
        }
    }
}

/// Serializer whose output is a `Value`.
pub struct Serializer;

impl serde::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SerializeVec;
    type SerializeTuple = SerializeVec;
    type SerializeTupleStruct = SerializeVec;
    type SerializeTupleVariant = Impossible<Value, Error>;
    type SerializeMap = Impossible<Value, Error>;
    type SerializeStruct = Impossible<Value, Error>;
    type SerializeStructVariant = Impossible<Value, Error>;

    #[inline]
    fn serialize_bool(self, value: bool) -> Result<Value> {
        Ok(Value::Boolean(value))
    }

    #[inline]
    fn serialize_i8(self, value: i8) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i16(self, value: i16) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    #[inline]
    fn serialize_i32(self, value: i32) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    fn serialize_i64(self, value: i64) -> Result<Value> {
        Ok(Value::Integer(value))
    }

    fn serialize_u8(self, value: u8) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    fn serialize_u16(self, value: u16) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    fn serialize_u32(self, value: u32) -> Result<Value> {
        self.serialize_i64(value as i64)
    }

    // FIXME: u64 -> i64
    fn serialize_u64(self, value: u64) -> Result<Value> {
        Ok(Value::Integer(value as i64))
    }

    fn serialize_f32(self, value: f32) -> Result<Value> {
        self.serialize_f64(f64::from(value))
    }

    fn serialize_f64(self, value: f64) -> Result<Value> {
        Ok(Value::Float(value.into()))
    }

    fn serialize_char(self, value: char) -> Result<Value> {
        Ok(Value::Character(value))
    }

    fn serialize_str(self, value: &str) -> Result<Value> {
        Ok(Value::String(value.into()))
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Value> {
        Ok(Value::String(str::from_utf8(value)?.into()))
    }

    fn serialize_none(self) -> Result<Value> {
        Ok(Value::Nil)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Value>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Value> {
        Ok(Value::Nil)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Value> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Value>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value>
    where
        T: Serialize,
    {
        todo!()
    }

    // serialization of compound types.

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SerializeVec {
            vec: Vec::with_capacity(len.unwrap_or(0)),
            as_list: false,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Ok(SerializeVec {
            vec: Vec::with_capacity(len),
            as_list: true,
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

pub struct SerializeVec {
    vec: Vec<Value>,
    as_list: bool,
}

impl serde::ser::SerializeSeq for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.vec.push(to_value(value)?);
        Ok(())
    }

    fn end(self) -> Result<Value> {
        if self.as_list {
            Ok(Value::List(self.vec))
        } else {
            Ok(Value::Vector(self.vec))
        }
    }
}

impl serde::ser::SerializeTuple for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for SerializeVec {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}

