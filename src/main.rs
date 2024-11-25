use std::io::stdin;
use std::ops::Not;

use spell_checker::{BasicSpellChecker, SpellChecker};
use spell_error::SpellError;
use wordifier::{SimpleWordifier, Wordifier};

mod spell_checker;
mod spell_error;
mod wordifier;

struct Wizard<S: SpellChecker, W: Wordifier> {
    s: S,
    w: W,
}

impl<S: SpellChecker, W: Wordifier> Wizard<S, W> {
    fn print_errors(&self, line_number: usize, line: String) {
        self.w
            .words(&line)
            .filter_map(|word| {
                self.s
                    .check(word)
                    .not()
                    .then_some(SpellError::new(&line, word))
            })
            .for_each(|error| println!("{line_number}:{error}"))
    }
}

fn main() -> std::io::Result<()> {
    let dictionary_path = std::env::args()
        .nth(1)
        .unwrap_or("dictionaries/small.txt".into());

    let dictionary = BasicSpellChecker::from_file(dictionary_path)?;

    let wizard = Wizard {
        s: dictionary,
        w: SimpleWordifier,
    };

    for (i, line) in stdin().lines().enumerate() {
        wizard.print_errors(i + 1, line?);
    }

    Ok(())
}
