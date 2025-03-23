mod alphabetic;
mod camel;
mod iter;

pub use alphabetic::Alphabetic;
pub use camel::CamelCase;

pub trait Wordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str>;
}
