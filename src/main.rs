use std::io::stdin;

use spell_checker::{BasicSpellChecker, LineMarker};
use wordifier::{SimpleWordifier, Wordifier};

mod spell_checker;
mod spell_error;
mod wordifier;

struct Wizard<S: LineMarker, W: Wordifier> {
    spelling: S,
    tokenizer: W,
}

impl<S: LineMarker, W: Wordifier> Wizard<S, W> {
    fn print_errors(&self, line_number: usize, line: &str) {
        self.tokenizer
            .words(&line)
            .filter_map(|word| self.spelling.mark(line, word))
            .for_each(|marked_line| println!("{line_number}:{marked_line}"))
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
