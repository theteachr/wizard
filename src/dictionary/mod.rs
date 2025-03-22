mod dumb;
mod set;

pub use set::Set;

pub trait Dictionary {
    fn contains(&self, word: &str) -> bool;
}
