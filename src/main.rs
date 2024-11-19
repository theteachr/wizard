use std::io::{self, stdin};

use spell_checker::{BasicSpellChecker, SpellChecker};
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
    fn print_errors(&self, line_number: usize, line: String) {
        self.w
            .wordify(&line)
            .filter(|word| !self.s.spell_check(word))
            .for_each(|error| println!("{line_number}:{}", SpellError::new(&line, error)))
    }

    fn run(self) -> io::Result<()> {
        for (i, line) in stdin().lines().enumerate() {
            self.print_errors(i + 1, line?);
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let dictionary_path = std::env::args()
        .nth(1)
        .unwrap_or("dictionaries/small.txt".to_owned());

    let dictionary = BasicSpellChecker::from_file(dictionary_path)?;

    Root {
        s: dictionary,
        w: SimpleWordifier,
    }
    .run()
}
