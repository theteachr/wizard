use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

pub struct BasicDictionary(HashSet<String>);

impl BasicDictionary {
    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let f = File::open(path)?;

        let words = std::io::BufReader::new(f)
            .lines()
            .filter_map(Result::ok)
            .map(|line| line.trim().to_lowercase())
            .collect();

        Ok(Self(words))
    }

    pub fn learn(&mut self, word: &str) {
        self.0.insert(word.to_lowercase());
    }
}

impl Default for BasicDictionary {
    fn default() -> Self {
        Self(HashSet::new())
    }
}

impl super::Dictionary for BasicDictionary {
    fn contains(&self, word: &str) -> bool {
        self.0.contains(&word.to_lowercase())
    }
}
