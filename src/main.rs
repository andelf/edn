use std::collections::HashMap;

use edn::edn;
use serde::Serialize;

#[derive(Serialize)]
pub struct Person {
    #[serde(rename = ":name")]
    name: String,
    #[serde(rename = ":age")]
    age: u32,
    #[serde(rename = ":email")]
    email: Vec<&'static str>,
}

fn main() {
    let fname = std::env::args().nth(1).unwrap();

    let input = std::fs::read_to_string(fname).unwrap();

    if let Err(err) = edn::parser::parse_edn(&input) {
        println!("Parse error {}", err);
    }

    let mut v = HashMap::new();
    v.insert(":x", 1);
    v.insert(":y", 2);
    v.insert(":z", 3);

    println!("=> {}", edn::to_value(v).unwrap());

    let tup = (1, 2, 3, "xdf");
    println!("=> {}", edn::to_value(tup).unwrap());

    let p = Person {
        name: "John".to_string(),
        age: 32,
        email: vec!["noreply@example.com", "me@noreply.com"],
    };
    println!("=> {}", edn::to_value(p).unwrap());

    println!("=> {}", edn!(false));
    println!("=> {}", edn!(true));
    println!("=> {}", edn!(nil));
    println!("=> {}", edn!([]));
    println!("=> {}", edn!(()));
    println!("=> {}", edn!([1, 2, 3]));
    println!("=> {}", edn!([1, nil, true, "asdfa", 9.9]));

    // println!("=> {}", edn!({:x}));
}
