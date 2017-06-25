struct HelloWorld;

impl HelloWorld {
    fn to_string (&self) -> &str { "Hello, World!!" }

    fn print(&self) {
        println!("{}", self.to_string());
    }
}

fn main() {
    (HelloWorld {}).print();
}
