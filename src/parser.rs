use pest::iterators::Pair;
use pest_derive::Parser;
use uuid::Uuid;

use crate::{error::Result, Map, Value};

#[derive(Parser)]
#[grammar = "grammars/edn.pest"] // relative to project `src`
struct EDNParser;

// input s must be a string literal, `"...."`
fn unescape_string(s: &str) -> String {
    if s.find('\\').is_none() {
        return s[1..s.len() - 1].to_string();
    }

    let mut result = String::with_capacity(s.len());
    let s = &s[1..s.len() - 1];

    let mut chars = s.char_indices();
    let mut start = 0;
    loop {
        match chars.next() {
            Some((i, c)) => {
                if c == '\\' {
                    if start != i {
                        result.push_str(&s[start..i]);
                    }
                    match s.as_bytes().get(i + 1) {
                        Some(b't') => result.push('\t'),
                        Some(b'r') => result.push('\r'),
                        Some(b'n') => result.push('\n'),
                        Some(b'f') => result.push('\x0C'),
                        Some(b'b') => result.push('\x08'),
                        Some(b'"') => result.push('"'),
                        Some(b'\\') => result.push('\\'),
                        Some(b'u') => {
                            let code = &s[i + 2..i + 6];
                            let code = u32::from_str_radix(code, 16).unwrap();
                            result.push(std::char::from_u32(code).unwrap());
                            chars.advance_by(4).unwrap();
                        }
                        Some(_) | None => {
                            panic!("Unexpected end of string")
                        }
                    }
                    chars.advance_by(1).unwrap();
                    start = chars.offset();
                }
            }
            None => {
                result.push_str(&s[start..]);
                break;
            }
        }
    }

    result
}

fn unescape_character(s: &str) -> char {
    match s {
        "newline" => '\n',
        "return" => '\r',
        "space" => ' ',
        "tab" => '\t',
        _ if s.len() == 1 => s.chars().next().unwrap(),
        _ if s.as_bytes()[0] == b'u' => {
            let code = u32::from_str_radix(&s[1..], 16).unwrap();
            std::char::from_u32(code).unwrap()
        }
        _ => panic!("Invalid character: {}", s),
    }
}

fn parse_value(pair: Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::nil => Value::Nil,
        Rule::boolean => Value::Boolean(pair.as_str() == "true"),
        Rule::integer => Value::Integer(pair.as_str().parse().unwrap()),
        Rule::float => Value::Float(pair.as_str().parse().unwrap()),
        Rule::string => Value::String(unescape_string(pair.as_str())),
        Rule::symbol => Value::Symbol(pair.as_str().into()),
        Rule::keyword => Value::Keyword(pair.as_str()[1..].into()),
        Rule::vector => Value::Vector(
            pair.into_inner()
                .filter(|p| p.as_rule() != Rule::discard)
                .map(parse_value)
                .collect(),
        ),
        Rule::list => Value::List(
            pair.into_inner()
                .filter(|p| p.as_rule() != Rule::discard)
                .map(parse_value)
                .collect(),
        ),
        Rule::set => Value::Set(
            pair.into_inner()
                .filter(|p| p.as_rule() != Rule::discard)
                .map(parse_value)
                .collect(),
        ),
        Rule::map => {
            let mut map = Map::new();
            let mut pairs = pair.into_inner();
            loop {
                match pairs.next_chunk() {
                    Ok([key, value]) => {
                        let key = parse_value(key);
                        let value = parse_value(value);
                        map.insert(key.try_into().unwrap(), value);
                    }
                    Err(it) => {
                        if it.count() != 0 {
                            panic!("Invalid map")
                        }
                        break;
                    }
                }
            }
            Value::Map(map)
        }
        Rule::character => Value::Character(unescape_character(&pair.as_str()[1..])),
        Rule::tagged => {
            let mut tagged = pair.into_inner();
            let tag = tagged.next().unwrap().as_str();

            if tag == "uuid" {
                let mut val = tagged.next().unwrap().as_str();
                val = &val[1..val.len() - 1];
                let uuid = Uuid::parse_str(val).unwrap();
                Value::Uuid(uuid)
            } else if tag == "inst" {
                let mut val = tagged.next().unwrap().as_str();
                val = &val[1..val.len() - 1];
                let inst = chrono::DateTime::parse_from_rfc3339(val).unwrap();
                Value::Instant(inst)
            } else {
                Value::Tagged(tag.into(), Box::new(parse_value(tagged.next().unwrap())))
            }
        }
        _ => {
            unreachable!()
        }
    }
}

pub fn parse_edn(input: &str) -> Result<Value> {
    use pest::Parser;

    let edn = EDNParser::parse(Rule::edn, input)?.next().unwrap();
    let val = parse_value(edn);

    Ok(val)
}
