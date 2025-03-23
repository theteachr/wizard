use std::io::stdin;

use wizard::lexicon::Set;
use wizard::wordifier::Alphabetic;
use wizard::Wizard;

fn main() -> std::io::Result<()> {
    let arg = std::env::args().nth(1);
    let path = arg.as_deref().unwrap_or("dictionaries/small.txt");

    let wizard = Wizard {
        lexicon: Set::from_file(path)?,
        wordifier: Alphabetic,
    };

    for (i, line) in stdin().lines().enumerate() {
        wizard
            .errors(line?.as_str())
            .for_each(|error| println!("{n}: {error}", n = i + 1));
    }

    Ok(())
}
