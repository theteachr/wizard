mod dumb;
mod set;

pub use dumb::Dumb;
pub use set::Set;

/// A `Lexicon` can be used to check whether a word is spelt correctly.
pub trait Lexicon {
    fn contains(&self, word: &str) -> bool;
}
