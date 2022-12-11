#![feature(iter_next_chunk, iter_intersperse)]
#![feature(test)]

extern crate test;

pub mod parser;
pub mod value;

pub use value::Value;

#[cfg(test)]
mod tests {
    use super::parser::parse_edn;
    use test::Bencher;

    #[test]
    fn test_parse() {
        let input = include_str!("../data/block.edn");

        assert!(parse_edn(input).is_ok());
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let input = include_str!("../data/block.edn");

        b.iter(|| {
            let _ = parse_edn(&input).unwrap();
        });
    }
}
