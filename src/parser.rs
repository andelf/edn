use std::collections::HashMap;

// use pest::Parser;
use pest::error::Error;
use pest::iterators::Pair;
use pest_derive::Parser;
use uuid::Uuid;

use crate::value::{Key as EDNKey, Value as EDNValue};

#[derive(Parser)]
#[grammar = "grammars/edn.pest"] // relative to project `src`
struct EDNParser;

fn unescape_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s[1..s.len()-1].chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('n') => result.push('\n'),
                Some('f') => result.push('\x0C'),
                Some('b') => result.push('\x08'),
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('u') => {
                    let mut code = String::new();
                    for _ in 0..4 {
                        code.push(chars.next().unwrap());
                    }
                    let code = u32::from_str_radix(&code, 16).unwrap();
                    result.push(std::char::from_u32(code).unwrap());
                }
                Some(c) => result.push(c),
                None => {
                    panic!("Unexpected end of string")
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn parse_value(pair: Pair<Rule>) -> EDNValue {
    match pair.as_rule() {
        Rule::nil => EDNValue::Nil,
        Rule::boolean => EDNValue::Boolean(pair.as_str() == "true"),
        Rule::integer => EDNValue::Integer(pair.as_str().parse().unwrap()),
        Rule::float => EDNValue::Float(pair.as_str().parse().unwrap()),
        Rule::string => EDNValue::String(unescape_string(pair.as_str())),
        Rule::symbol => EDNValue::Symbol(pair.as_str().to_string()),
        Rule::keyword => EDNValue::Keyword(pair.as_str()[1..].to_string()),
        Rule::vector => EDNValue::Vector(pair.into_inner().map(parse_value).collect()),
        Rule::list => EDNValue::List(pair.into_inner().map(parse_value).collect()),
        Rule::set => EDNValue::Set(pair.into_inner().map(parse_value).collect()),
        Rule::map => {
            let mut map: HashMap<EDNKey, EDNValue> = HashMap::new();
            for value_pairs in pair.into_inner() {
                let mut value_pairs = value_pairs.into_inner();
                while let Ok([key, value]) = value_pairs.next_chunk() {
                    let key = parse_value(key);
                    let value = parse_value(value);
                    map.insert(key.try_into().unwrap(), value);
                }
            }

            EDNValue::Map(map)
        }
        Rule::character => EDNValue::Character(pair.as_str().chars().next().unwrap()),
        Rule::tagged => {
            let mut tagged = pair.into_inner();
            let tag = tagged.next().unwrap().as_str();

            if tag == "uuid" {
                let val = tagged.next().unwrap().as_str();
                let uuid = Uuid::parse_str(val.trim_matches('"')).unwrap();
                EDNValue::Uuid(uuid)
            } else {
                EDNValue::Tagged(
                    tag.to_string(),
                    Box::new(parse_value(tagged.next().unwrap())),
                )
            }
        }
        _ => {
            unimplemented!("{}", pair);
        }
    }
}

pub fn parse_edn(input: &str) -> Result<(), Error<Rule>> {
    use pest::Parser;

    let edn = EDNParser::parse(Rule::edn, input)?.next().unwrap();

    let val = parse_value(edn);

    println!("{}", val);

    Ok(())
}
