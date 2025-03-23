mod dumb;
mod set;

pub use set::Set;

// XXX: A dictionary is not the best of words to describe this trait.

pub trait Dictionary {
    fn contains(&self, word: &str) -> bool;
}
