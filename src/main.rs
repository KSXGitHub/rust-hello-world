pub mod input;
pub mod factorial;
pub mod fibonacci;
pub mod types;

pub use input::tuple::*;
pub use factorial::*;
pub use fibonacci::*;

pub fn main () {
    let (begin, end) = input("From", "To");

    let vecres = (begin..end).collect::<Vec<_>>();
    println!("Vector: {:?}", vecres);

    let sqrres = (begin..end).map(|x| x * x).collect::<Vec<_>>();
    println!("Square: {:?}", sqrres);

    let facres = ((begin as u8)..(end as u8)).map(fac).collect::<Vec<_>>();
    println!("Factorial: {:?}", facres);

    let fibres = ((begin as u16)..(end as u16)).map(fib).collect::<Vec<_>>();
    println!("Fibonacci: {:?}", fibres);
}
