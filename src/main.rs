mod core;
mod error;

use std::io::stdin;

use wizard::dictionary::BasicDictionary;
use wizard::wordifier::BasicWordifier;

use core::Wizard;

fn main() -> std::io::Result<()> {
    let arg = std::env::args().nth(1);
    let path = arg.as_deref().unwrap_or("dictionaries/small.txt");

    let wizard = Wizard {
        dictionary: BasicDictionary::from_file(path)?,
        wordifier: BasicWordifier,
    };

    for (i, line) in stdin().lines().enumerate() {
        wizard
            .errors(line?.as_str())
            .for_each(|error| println!("{n}:{error}", n = i + 1));
    }

    Ok(())
}
