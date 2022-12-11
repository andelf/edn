#![feature(iter_next_chunk, iter_intersperse)]
#![feature(test)]

extern crate test;

pub mod parser;
pub mod symbol;
pub mod value;
pub mod error;
mod macros;
// pub mod ser;

pub use value::Value;
pub use symbol::Symbol;

pub use value::to_value;



#[cfg(test)]
mod tests {
    use super::parser::parse_edn;
    use test::Bencher;

    const INPUT: &str = include_str!("../data/docs.edn");

    #[test]
    fn test_parse() {
        assert!(parse_edn(INPUT).is_ok());
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse_edn(INPUT).unwrap();
        });
    }

    #[bench]
    fn bench_dump(b: &mut Bencher) {
        let v = parse_edn(INPUT).unwrap();

        b.iter(|| {
            let _ = v.to_string();
        });
    }

    #[bench]
    fn bench_compare(b: &mut Bencher) {
        let v1 = parse_edn(INPUT).unwrap();
        let v2 = parse_edn(INPUT).unwrap();

        b.iter(|| {
            assert_eq!(v1, v2);
        });
    }
}
