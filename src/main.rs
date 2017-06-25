extern crate hello_world;
pub use hello_world::lib::HelloWorld;
pub use self::factorial as fac;

pub fn factorial(n: u8) -> u64 {
    let nn = n as u64;
    if nn > 1 {
        nn * fac(n - 1)
    } else {
        1
    }
}

fn main() {
    for n in 0..10 {
        println!("{}! = {}", n, fac(n));
    }
    HelloWorld::create_printer()();
}
