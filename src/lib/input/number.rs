use lib::types::Number;
use std::io::*;

/// # Examples
/// ```rust
/// # fn main() {
///     # use lib::input::number;
///     let mynum = number::input("Input a number");
///     # println!("Entered number: {?:}", mynum);
/// # }
/// ```
pub fn input(question: &str) -> Number {
    print!("{}: ", question);
    stdout().flush().ok().expect("Failed to flush to stdout.");

    let mut s = String::new();
    stdin().read_line(&mut s).ok().expect("Failed to read the line.");

    s.trim().parse().ok().expect("Not a number")
}
