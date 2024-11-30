use std::io::stdin;

use spell_checker::{BasicSpellChecker, SpellChecker};
use spell_error::SpellError;
use wordifier::{SimpleWordifier, Wordifier};

mod spell_checker;
mod spell_error;
mod wordifier;

struct Wizard<S: SpellChecker, W: Wordifier> {
    spelling: S,
    tokenizer: W,
}

impl<S: SpellChecker, W: Wordifier> Wizard<S, W> {
    fn print_errors(&self, line_number: usize, line: &str) {
        self.tokenizer
            .words(&line)
            .filter(|word| !self.spelling.is_valid(word))
            .for_each(|typo| println!("{line_number}:{}", SpellError::new(line, typo)))
    }
}

fn main() -> std::io::Result<()> {
    let dictionary_path = std::env::args()
        .nth(1)
        .unwrap_or("dictionaries/small.txt".into());

    let dictionary = BasicSpellChecker::from_file(dictionary_path)?;

    let wizard = Wizard {
        spelling: dictionary,
        tokenizer: SimpleWordifier,
    };

    for (i, line) in stdin().lines().enumerate() {
        wizard.print_errors(i + 1, &line?);
    }

    Ok(())
}
