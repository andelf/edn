use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use chrono::{DateTime, FixedOffset, SecondsFormat};
use ordered_float::OrderedFloat;
use uuid::Uuid;

/// Represents any valid EDN value.
#[derive(Clone, Eq, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    Float(OrderedFloat<f64>),
    String(String),
    Symbol(String),
    Keyword(String),
    Vector(Vec<Value>),
    List(Vec<Value>),
    Set(HashSet<Value>),
    Map(HashMap<Key, Value>),
    Instant(DateTime<FixedOffset>),
    Uuid(Uuid),
    Character(char),
    Tagged(String, Box<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            // TODO: impl using writer
        }
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(v) => write!(f, "{}", v),
            Value::String(s) => write!(f, "{:?}", s),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Keyword(s) => write!(f, ":{}", s),
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
            Value::Character(c) => write!(f, "\\{}", c),
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
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Key {
    Keyword(String),
    String(String),
    Symbol(String),
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Key::Keyword(s) => write!(f, ":{}", s),
            Key::String(s) => write!(f, "{:?}", s),
            Key::Symbol(s) => write!(f, "{}", s),
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
            _ => Err(format!("Invalid key: {}", value)),
        }
    }
}
