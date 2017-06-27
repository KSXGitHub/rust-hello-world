extern crate rand;

pub fn get(begin: u32, end: u32 = 255) -> u32 {
    rand::thread_rng().gen_range(begin, end)
}
