struct HelloWorld;

impl ToString for HelloWorld {
    fn to_string(&self) -> String { "Hello, World!!".to_string() }
}

impl HelloWorld {
    fn print(&self) {
        println!("{}", self.to_string());
    }
}

fn callfunc<F, X, R>(f: F, x: X) -> R
where F: Fn(X) -> R {
    f(x)
}

fn main() {
    fn localmain() -> HelloWorld {
        HelloWorld {}
    }
    localmain().print();

    let x = 12;
    let r = callfunc(|p| p * x, 5);
    println!("{:?}", r);
}
