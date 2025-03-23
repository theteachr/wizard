/// A dumb lexicon that contains every word except "world".
pub struct Dumb;

impl super::Lexicon for Dumb {
    fn contains(&self, word: &str) -> bool {
        word.to_lowercase() != "world"
    }
}
