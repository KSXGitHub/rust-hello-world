
/// # Effects
///
/// * Takes an integer (n: u16)
/// * Return fibonacci at position 'n' (F[n])
///
/// # Fibonacci definition
///
/// * F[0] = 0, F[1] = 1
/// * ∀ n ≥ 2: F[n] = F[n - 1] + F[n - 2]
///
/// # Examples
/// ```rust
/// # fn main() {
///     # use lib::fibonacci::fibonacci;
///     for let n in 0..10 {
///         println!("F[{}] = {}", n, fibonacci(n));
///     }
/// # }
/// ```
pub fn fibonacci (n: u16) -> u64 {
    match n {
        0 | 1 => 1u64,
        2 => 2u64,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

/// Alias to `fibonacci`
pub use self::fibonacci as fib;
