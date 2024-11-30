use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub trait SpellChecker {
    fn is_valid(&self, word: &str) -> bool;
}

pub struct BadSpellChecker;

impl SpellChecker for BadSpellChecker {
    fn is_valid(&self, word: &str) -> bool {
        return word.to_lowercase() != "world";
    }
}

pub struct BasicSpellChecker(HashSet<String>);

impl BasicSpellChecker {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let words = std::io::BufReader::new(f)
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.trim().to_lowercase())
            .collect();

        Ok(Self(words))
    }
}

impl SpellChecker for BasicSpellChecker {
    fn is_valid(&self, word: &str) -> bool {
        self.0.contains(&word.to_lowercase())
    }
}
