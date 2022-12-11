use std::borrow::Cow;

use super::{Key, Value};

use crate::map::Map;

macro_rules! from_integer {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Value {
                fn from(n: $ty) -> Self {
                    Value::Integer(n as i64)
                }
            }
        )*
    };
}

from_integer! {
    i8 i16 i32 i64 isize
    u8 u16 u32 u64 usize
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        From::from(f as f64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f.into())
    }
}

impl From<bool> for Value {
    fn from(f: bool) -> Self {
        Value::Boolean(f)
    }
}

impl From<String> for Value {
    fn from(f: String) -> Self {
        Value::String(f)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(f: &str) -> Self {
        Value::String(f.to_string())
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    fn from(f: Cow<'a, str>) -> Self {
        Value::String(f.into_owned())
    }
}

impl From<Map<Key, Value>> for Value {
    fn from(f: Map<Key, Value>) -> Self {
        Value::Map(f)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(f: Vec<T>) -> Self {
        Value::Vector(f.into_iter().map(Into::into).collect())
    }
}

impl<'a, T: Clone + Into<Value>> From<&'a [T]> for Value {
    fn from(f: &'a [T]) -> Self {
        Value::Vector(f.iter().cloned().map(Into::into).collect())
    }
}

impl<T: Into<Value>> FromIterator<T> for Value {
    /// Convert an iteratable type to a `Value`
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Value::Vector(iter.into_iter().map(Into::into).collect())
    }
}

impl<K: Into<Key>, V: Into<Value>> FromIterator<(K, V)> for Value {
    /// Convert an iteratable pair type to a `Value`
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Value::Map(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl From<()> for Value {
    fn from((): ()) -> Self {
        Value::Nil
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => Value::Nil,
            Some(value) => Into::into(value),
        }
    }
}
