pub use std::fmt;
pub use std::result;
pub use lib::traits;

struct Printer;

impl fmt::Display for Printer {
    fn fmt(&self, _: &mut fmt::Formatter<>) -> result::Result<(), fmt::Error> {
        write!("{}", "Hello, World")
    }
}
