mod basic;
mod dumb;

pub use basic::BasicDictionary;

pub trait Dictionary {
    fn contains(&self, word: &str) -> bool;
}
