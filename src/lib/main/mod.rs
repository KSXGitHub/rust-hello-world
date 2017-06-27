use super::input::tuple::*;
use super::factorial::*;
use super::fibonacci::*;

/// # main
/// The main function
/// Will be called by `main` in `main.rs`
pub fn main () {
    println!("Enter the range of input:");
    let (begin, end) = input("* From", "* To");

    let vecres = (begin..end).collect::<Vec<_>>();
    println!("Vector: {:?}", vecres);

    let sqrres = (begin..end).map(|x| x * x).collect::<Vec<_>>();
    println!("Square: {:?}", sqrres);

    let facres = ((begin as u8)..(end as u8)).map(fac).collect::<Vec<_>>();
    println!("Factorial: {:?}", facres);

    let fibres = ((begin as u16)..(end as u16)).map(fib).collect::<Vec<_>>();
    println!("Fibonacci: {:?}", fibres);
}
