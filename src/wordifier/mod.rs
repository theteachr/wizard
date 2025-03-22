mod basic;
mod camel;
mod iter;

pub use basic::Alphabetic;
pub use camel::CamelCase;

pub trait Wordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str>;
}
