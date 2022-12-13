use std::{borrow::Cow, collections::HashSet, fmt::Display};

use chrono::{DateTime, FixedOffset, SecondsFormat};
use ordered_float::OrderedFloat;
use serde::Serialize;
use uuid::Uuid;

use self::ser::Serializer;
use crate::error::Error;
use crate::symbol::Symbol;
use crate::Map;

mod from;
mod ser;

/// Represents any valid EDN value.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Symbol(Symbol),
    Keyword(Symbol),
    Vector(Vec<Value>),
    List(Vec<Value>),
    Set(HashSet<Value>),
    Map(Map<Key, Value>),
    Instant(DateTime<FixedOffset>),
    Uuid(Uuid),
    Character(char),
    Tagged(Symbol, Box<Value>),
}

pub fn to_value<T>(value: T) -> Result<Value, Error>
where
    T: Serialize,
{
    value.serialize(Serializer)
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            simple_pprint(f, self, 0)?;
            return Ok(());
        }
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(v) => write!(f, "{}", v),
            Value::String(s) => write!(f, "{:?}", s),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Keyword(s) => write!(f, "{}", s),
            Value::Vector(v) => write!(
                f,
                "[{}]",
                v.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Value::List(v) => write!(
                f,
                "({})",
                v.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Value::Set(s) => write!(
                f,
                "#{{{}}}",
                s.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Value::Map(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    write!(f, "{} {}", k, v)?;
                    if i < m.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            Value::Instant(i) => write!(
                f,
                "#inst \"{}\"",
                i.to_rfc3339_opts(SecondsFormat::Millis, true)
            ),
            Value::Uuid(u) => write!(f, "#uuid \"{}\"", u),
            Value::Character(c) => write!(f, "\\{}", escape_character(c)),
            Value::Tagged(t, v) => write!(f, "#{} {}", t, v),
        }
    }
}

impl std::hash::Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Nil => state.write_u8(0),
            Value::Boolean(b) => {
                state.write_u8(1);
                b.hash(state);
            }
            Value::Integer(i) => {
                state.write_u8(2);
                i.hash(state);
            }
            Value::Float(f) => {
                state.write_u8(3);
                OrderedFloat(*f).hash(state);
            }
            Value::String(s) => {
                state.write_u8(4);
                s.hash(state);
            }
            Value::Symbol(s) => {
                state.write_u8(5);
                s.hash(state);
            }
            Value::Keyword(s) => {
                state.write_u8(6);
                s.hash(state);
            }
            Value::Vector(v) => {
                state.write_u8(7);
                v.hash(state);
            }
            Value::List(v) => {
                state.write_u8(8);
                v.hash(state);
            }
            Value::Set(s) => {
                state.write_u8(9);
                for v in s {
                    v.hash(state);
                }
            }
            Value::Map(m) => {
                state.write_u8(10);
                for (k, v) in m {
                    k.hash(state);
                    v.hash(state);
                }
            }
            Value::Instant(i) => {
                state.write_u8(11);
                i.hash(state);
            }
            Value::Uuid(u) => {
                state.write_u8(12);
                u.hash(state);
            }
            Value::Character(c) => {
                state.write_u8(13);
                c.hash(state);
            }
            Value::Tagged(t, v) => {
                state.write_u8(14);
                t.hash(state);
                v.hash(state);
            }
        }
    }
}

/// Represents any valid EDN key.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
    Keyword(Symbol),
    String(String),
    Symbol(Symbol),
    Integer(i64),
    Boolean(bool),
    Character(char),
    Uuid(Uuid),
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Key::Keyword(s) => write!(f, "{}", s),
            Key::String(s) => write!(f, "{:?}", s),
            Key::Symbol(s) => write!(f, "{}", s),
            Key::Integer(i) => write!(f, "{}", i),
            Key::Boolean(b) => write!(f, "{}", b),
            Key::Character(c) => write!(f, "\\{}", escape_character(c)),
            Key::Uuid(u) => write!(f, "#uuid \"{}\"", u),
        }
    }
}

fn simple_pprint(
    f: &mut std::fmt::Formatter,
    value: &Value,
    indentation: usize,
) -> std::fmt::Result {
    f.write_str(&" ".repeat(indentation * 2))?;
    match value {
        Value::Nil => writeln!(f, "nil"),
        Value::Boolean(b) => writeln!(f, "{}", b),
        Value::Integer(i) => writeln!(f, "{}", i),
        Value::Float(v) => writeln!(f, "{}", v),
        Value::String(s) => writeln!(f, "{:?}", s),
        Value::Symbol(s) => writeln!(f, "{}", s),
        Value::Keyword(s) => writeln!(f, "{}", s),
        Value::Vector(val) => {
            writeln!(f, "[")?;
            for (i, v) in val.iter().enumerate() {
                simple_pprint(f, v, indentation + 1)?;
            }
            f.write_str(&" ".repeat(indentation * 2))?;
            writeln!(f, "]")
        }
        Value::List(val) => {
            writeln!(f, "(")?;
            for (i, v) in val.iter().enumerate() {
                simple_pprint(f, v, indentation + 1)?;
            }
            f.write_str(&" ".repeat(indentation * 2))?;
            writeln!(f, ")")
        }
        Value::Set(s) => {
            writeln!(f, "#{{")?;
            for (i, v) in s.iter().enumerate() {
                simple_pprint(f, v, indentation + 1)?;
            }
            f.write_str(&" ".repeat(indentation * 2))?;
            writeln!(f, "}}")
        }
        Value::Map(m) => {
            writeln!(f, "{{")?;
            for (i, (k, v)) in m.iter().enumerate() {
                f.write_str(&" ".repeat(indentation * 2 + 2))?;
                writeln!(f, "{}", k)?;
                simple_pprint(f, v, indentation + 2)?;
            }
            f.write_str(&" ".repeat(indentation * 2))?;
            writeln!(f, "}}")
        }
        Value::Instant(i) => writeln!(
            f,
            "#inst \"{}\"",
            i.to_rfc3339_opts(SecondsFormat::Millis, true)
        ),
        Value::Uuid(u) => writeln!(f, "#uuid \"{}\"", u),
        Value::Character(c) => writeln!(f, "\\{}", escape_character(c)),
        Value::Tagged(t, v) => writeln!(f, "#{} {}", t, v),
    }
}

// Use keyward as a default key
impl<T: AsRef<str>> From<T> for Key {
    fn from(s: T) -> Self {
        if s.as_ref().as_bytes()[0] == b':' {
            Key::Keyword(s.as_ref().into())
        } else {
            Key::Symbol(s.as_ref().into())
        }
    }
}

impl TryFrom<Value> for Key {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Keyword(s) => Ok(Key::Keyword(s)),
            Value::String(s) => Ok(Key::String(s)),
            Value::Symbol(s) => Ok(Key::Symbol(s)),
            Value::Integer(i) => Ok(Key::Integer(i)),
            Value::Boolean(b) => Ok(Key::Boolean(b)),
            Value::Character(c) => Ok(Key::Character(c)),
            Value::Uuid(u) => Ok(Key::Uuid(u)),
            _ => Err(format!("Invalid key: {}", value)),
        }
    }
}

fn escape_character(c: &char) -> Cow<'static, str> {
    match c {
        '\n' => "newline".into(),
        '\r' => "return".into(),
        ' ' => "space".into(),
        '\t' => "tab".into(),
        c => c.to_string().into(),
    }
}
