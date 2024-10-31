pub trait SpellChecker {
    fn spell_check(&self, word: &str) -> bool;
}

pub struct BadSpellChecker;

impl SpellChecker for BadSpellChecker {
    fn spell_check(&self, word: &str) -> bool {
        return word.to_lowercase() != "world";
    }
}
