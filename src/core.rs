use wizard::dictionary::Dictionary;
use wizard::wordifier::Wordifier;

use crate::error::Error;

pub struct Wizard<D: Dictionary, W: Wordifier> {
    pub dictionary: D,
    pub wordifier: W,
}

impl<S: Dictionary, W: Wordifier> Wizard<S, W> {
    pub fn errors<'a>(&'a self, line: &'a str) -> impl Iterator<Item = Error<'a>> {
        self.wordifier
            .words(line)
            .filter(|word| !self.dictionary.contains(word))
            .map(|typo| Error::new(line, typo))
    }
}
