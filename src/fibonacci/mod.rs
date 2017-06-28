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

pub use self::fibonacci as fib;
