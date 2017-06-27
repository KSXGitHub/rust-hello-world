use lib::types::Number;
use super::number;

pub fn input(a: &str, b: &str) -> (Number, Number) {
    (number::input(a), number::input(b))
}
