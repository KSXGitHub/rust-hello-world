struct HelloWorld;

impl ToString for HelloWorld {
    fn to_string(&self) -> String { "Hello, World!!".to_string() }
}

impl HelloWorld {
    fn print(&self) {
        println!("{}", self.to_string());
    }
}

fn main() {
    fn localmain() -> HelloWorld {
        HelloWorld {}
    }
    localmain().print();
}
