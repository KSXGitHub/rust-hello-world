pub fn fibonacci (n: u16) -> u64 {
    match n {
        0 | 1 => 1u64,
        2 => 2u64,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub use self::fibonacci as fib;
