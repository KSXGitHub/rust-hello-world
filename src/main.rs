struct HelloWorld;

impl ToString for HelloWorld {
    fn to_string(&self) -> String { "Hello, World!!".to_string() }
}

impl HelloWorld {
    fn print(&self) {
        println!("{}", self.to_string());
    }

    fn create_printer() -> Box<Fn()> {
        Box::new(|| (HelloWorld {}).print())
    }
}

fn main() {
    HelloWorld::create_printer()();
}
