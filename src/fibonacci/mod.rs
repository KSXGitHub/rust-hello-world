pub fn fibonacci (n: u16) -> u64 {
    let n = n as u64;
    if n < 2 { return n }

    let (mut x, mut y) = (0, 1);

    for _ in 1..n {
        let ty = y;
        y += x;
        x = ty;
    }

    y
}

pub use self::fibonacci as fib;
