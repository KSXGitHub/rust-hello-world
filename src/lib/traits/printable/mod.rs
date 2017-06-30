pub use std::fmt;

pub trait Printable {
    type Type: fmt::Display;

    fn print(&self, val: &Self::Type);
    fn println(&self, val: &Self::Type);
}
