extern crate hello_world;
pub use hello_world::swap::swap;

pub fn main() {
    let (mut a, mut b) = ("hello", "world");
    println!("Before swap(a, b): {:?}", [a, b]);
    swap(&mut a, &mut b);
    println!("After swap(a, b): {:?}", [a, b]);
}
