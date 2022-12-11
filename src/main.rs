fn main() {
    let fname = std::env::args().nth(1).unwrap();

    let input = std::fs::read_to_string(fname).unwrap();

    if let Err(err) = edn::parser::parse_edn(&input) {
        println!("Parse error {}", err);
    }
}
