pub mod printable;

pub trait Printable: printable::Printable {
    type Type;
}
