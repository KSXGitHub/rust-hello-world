pub struct HelloWorld;

impl ToString for HelloWorld {
    fn to_string(&self) -> String { "Hello, World!!".to_string() }
}

impl HelloWorld {
    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    pub fn create_printer() -> Box<Fn()> {
        Box::new(|| (HelloWorld {}).print())
    }
}
