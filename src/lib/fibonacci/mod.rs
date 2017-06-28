
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
    if n == 0 { return 0 }
    if n == 1 { return 1 }

    let (mut x, mut y) = (0, 1);

    for _ in 1..(n as u64) {
        let ty = y;
        y += x;
        x = ty;
    }

    y
}

/// Alias to `fibonacci`
pub use self::fibonacci as fib;
