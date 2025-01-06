use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::ops::Not;
use std::path::Path;

pub struct SpellError<'a> {
    line: &'a str,
    error: &'a str,
}

impl std::fmt::Display for SpellError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.error.as_ptr() as usize - self.line.as_ptr() as usize;
        write!(
            f,
            "{}~~{}~~{}",
            &self.line[..start],
            &self.error,
            &self.line[start + self.error.len()..],
        )
    }
}

pub trait SpellChecker {
    fn is_valid(&self, word: &str) -> bool;
}

pub trait LineMarker {
    fn mark<'a>(&self, line: &'a str, word: &'a str) -> Option<SpellError<'a>>;
}

impl<T: SpellChecker> LineMarker for T {
    fn mark<'a>(&self, line: &'a str, word: &'a str) -> Option<SpellError<'a>> {
        self.is_valid(word)
            .not()
            .then(|| SpellError { line, error: word })
    }
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
