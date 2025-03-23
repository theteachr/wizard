mod alphabetic;
mod camel;
mod iter;

pub use alphabetic::Alphabetic;
pub use camel::CamelCase;

/// A `Wordifier` provides an iterator over the words in a line of text.
pub trait Wordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str>;
}

// XXX: Maybe this should not exist.
// We could implement `Iterator` on the types implementing `Wordifier` instead.
