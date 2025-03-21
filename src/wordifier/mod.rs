mod basic;
mod camel;
mod iter;

pub use basic::BasicWordifier;
pub use camel::CamelCaseWordifier;

pub trait Wordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str>;
}
