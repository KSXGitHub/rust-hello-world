
/// # Effects
///
/// * Takes an integer (n: u16)
/// * Return factorial of 'n' (n!)
///
/// # Factorial definition
///
/// * 0! = 1
/// * ∀ n ≥ 1: n! = n ❌ (n - 1)!
///
/// # Examples
/// ```rust
/// # fn main() {
///     # use lib::factorial::factorial;
///     for let n in 0..10 {
///         println!("F[{}] = {}", n, factorial(n));
///     }
/// # }
/// ```
pub fn factorial (n: u8) -> u64 {
    let mut product: u64 = 1;
    for x in 2..n { product *= x as u64 }
    product
}

pub use self::factorial as fac;
