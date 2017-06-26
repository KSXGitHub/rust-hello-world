extern crate hello_world;
use hello_world::factorial::fac;
use hello_world::fibonacci::fib;

pub fn main () {
    let (begin, end) = (0 as u8, 16 as u8);

    let vecres = (begin..end).collect::<Vec<_>>();
    println!("Vector: {:?}", vecres);

    let sqrres = (begin..end).map(|x| x * x).collect::<Vec<_>>();
    println!("Square: {:?}", sqrres);

    let facres = (begin..end).map(fac).collect::<Vec<u64>>();
    println!("Factorial: {:?}", facres);

    let fibres = ((begin as u16)..(end as u16)).map(fib).collect::<Vec<u64>>();
    println!("Fibonacci: {:?}", fibres);
}
