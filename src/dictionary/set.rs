use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

/// A hash set implementing the `Dictionary` trait.
pub struct Set(HashSet<String>);

impl Set {
    pub fn new<T: IntoIterator<Item: AsRef<str>>>(words: T) -> Self {
        Self(
            words
                .into_iter()
                .map(|word| word.as_ref().to_lowercase())
                .collect(),
        )
    }

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

impl Default for Set {
    fn default() -> Self {
        Self(HashSet::new())
    }
}

impl super::Dictionary for Set {
    fn contains(&self, word: &str) -> bool {
        self.0.contains(&word.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use crate::dictionary::Dictionary;

    use super::Set;

    fn small_dictionary() -> Set {
        Set::new(["a", "an", "Bird"])
    }

    #[test]
    fn learned_spell() {
        let d = small_dictionary();

        assert!(d.contains("a"));
        assert!(d.contains("an"));
        assert!(d.contains("Bird"));
    }

    #[test]
    fn did_not_learn_misspelled() {
        assert!(!small_dictionary().contains("birdy"));
    }

    #[test]
    fn learned_spell_insensitively() {
        let d = small_dictionary();

        assert!(d.contains("A"));
        assert!(d.contains("aN"));
        assert!(d.contains("Bird"));
    }
}
