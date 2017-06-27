use lib::types::Number;
use super::number;

/// # Effects
///
/// * Ask two integers from stdin with two questions
/// * Return a tuple contains these integers
///
/// # Examples
///
/// ```rust
/// # fn main() {
///     # use lib::input::tuple;
///     let (first, second) = tuple::input(
///         "Enter the first number", // question 1 for integer 'first'
///         "Enter the second number" // question 2 for integer 'second'
///     )
///     # println!("Entered tuple: {:?}", (first, second));
/// # }
/// ```
pub fn input(a: &str, b: &str) -> (Number, Number) {
    (number::input(a), number::input(b))
}
