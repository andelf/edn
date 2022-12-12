#![feature(
    iter_next_chunk,
    iter_intersperse,
    char_indices_offset,
    iter_advance_by
)]
#![feature(test)]

extern crate test;

pub mod error;
mod macros;
pub mod map;
pub mod parser;
pub mod symbol;
pub mod value;
// pub mod ser;

pub use map::Map;
pub use symbol::Symbol;
pub use value::Value;

pub use value::to_value;

#[cfg(test)]
mod tests {
    use super::parser::parse_edn;

    const INPUT: &str = include_str!("../data/block.edn");

    #[test]
    fn test_parse() {
        assert!(parse_edn(INPUT).is_ok());
    }
}
