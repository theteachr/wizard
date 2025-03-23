use crate::lexicon::Lexicon;
use crate::wordifier::Wordifier;

use crate::error::Error;

pub struct Wizard<L: Lexicon, W: Wordifier> {
    pub lexicon: L,
    pub wordifier: W,
}

impl<S: Lexicon, W: Wordifier> Wizard<S, W> {
    pub fn errors<'a>(&'a self, line: &'a str) -> impl Iterator<Item = Error<'a>> {
        self.wordifier
            .words(line)
            .filter(|word| !self.lexicon.contains(word))
            .map(|typo| Error::new(line, typo))
    }
}
