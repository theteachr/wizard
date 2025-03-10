mod basic;

pub use basic::BasicWordifier;

pub trait Wordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str>;
}
