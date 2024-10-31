use std::io::stdin;
use std::ops::Not;

use spell_checker::{BadSpellChecker, SpellChecker};
use spell_error::SpellError;
use wordifier::{SimpleWordifier, Wordifier};

mod spell_checker;
mod spell_error;
mod wordifier;

struct Root<S: SpellChecker, W: Wordifier> {
    s: S,
    w: W,
}

impl<S: SpellChecker, W: Wordifier> Root<S, W> {
    fn errors<'a>(&self, line: &'a str) -> Vec<&'a str> {
        self.w
            .wordify(&line)
            .filter_map(|word| self.s.spell_check(word).not().then_some(word))
            .collect()
    }

    fn run(self) {
        stdin()
            .lines()
            .enumerate()
            .filter_map(|(ln, line)| line.ok().map(|line| (ln + 1, line)))
            .for_each(|(ln, ref line)| {
                self.errors(line)
                    .into_iter()
                    .for_each(|error| println!("{ln}:{}", SpellError::new(line, error)))
            });
    }
}

fn main() {
    Root {
        s: BadSpellChecker,
        w: SimpleWordifier,
    }
    .run()
}
