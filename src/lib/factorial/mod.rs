pub fn factorial (n: u8) -> u64 {
    let mut product: u64 = 1;
    for x in 2..n { product *= x as u64 }
    product
}

pub use self::factorial as fac;
