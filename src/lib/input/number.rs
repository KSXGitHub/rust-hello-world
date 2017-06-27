use types::Number;
use std::io::*;

pub fn input(question: &str) -> Number {
    print!("{}: ", question);
    stdout().flush().ok().expect("Failed to flush to stdout.");

    let mut s = String::new();
    stdin().read_line(&mut s).ok().expect("Failed to read the line.");

    s.trim().parse().ok().expect("Not a number")
}
