use std::io::stdin;
use std::ops::Not;

#[derive(Debug)]
struct SpellError<'a> {
    line: &'a str,
    error: &'a str,
}

impl std::fmt::Display for SpellError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.error.as_ptr() as usize - self.line.as_ptr() as usize;
        write!(
            f,
            "{}~~{}~~{}",
            &self.line[..start],
            &self.error,
            &self.line[start + self.error.len()..],
        )
    }
}

trait Wordifier {
    fn wordify<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str>;
}

struct Root<S: SpellChecker, W: Wordifier> {
    spell_checker: S,
    wordifier: W,
}

trait SpellChecker {
    fn spell_check(&self, word: &str) -> bool;
}

impl<S: SpellChecker, W: Wordifier> Root<S, W> {
    fn new(spell_checker: S, wordifier: W) -> Self {
        Self {
            spell_checker,
            wordifier,
        }
    }

    fn errors<'a>(&self, line: &'a str) -> Vec<&'a str> {
        self.wordifier
            .wordify(&line)
            .filter_map(|word| self.spell_checker.spell_check(word).not().then_some(word))
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
                    .for_each(|error| println!("{ln}:{}", SpellError { line, error }))
            });
    }
}

struct BadSpellChecker;
struct SimpleWordifier;

impl Wordifier for SimpleWordifier {
    fn wordify<'a, 'b>(&'a self, line: &'b str) -> impl Iterator<Item = &'b str> {
        line.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
    }
}

impl SpellChecker for BadSpellChecker {
    fn spell_check(&self, word: &str) -> bool {
        return word.to_lowercase() != "world";
    }
}

fn main() {
    Root::new(BadSpellChecker, SimpleWordifier).run()
}
