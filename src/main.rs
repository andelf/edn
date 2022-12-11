
use edn::edn;

fn main() {
    let fname = std::env::args().nth(1).unwrap();

    let input = std::fs::read_to_string(fname).unwrap();

    if let Err(err) = edn::parser::parse_edn(&input) {
        println!("Parse error {}", err);
    }


    println!("=> {}", edn!(false));
    println!("=> {}", edn!(true));
    println!("=> {}", edn!(nil));
    println!("=> {}", edn!([]));
    println!("=> {}", edn!(()));
    println!("=> {}", edn!([1, 2, 3]));
    println!("=> {}", edn!([1, nil, true, "asdfa", 9.9]));

    println!("=> {}", edn!(:x));

}
