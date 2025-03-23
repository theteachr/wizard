mod dumb;
mod set;

pub use set::Set;

// XXX: A dictionary is not the best of words to describe this trait.

/// A `Dictionary` can tell whether a word is correctly spelt.
pub trait Dictionary {
    fn contains(&self, word: &str) -> bool;
}
