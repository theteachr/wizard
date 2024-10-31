use std::io::stdin;
use std::ops::Not;

#[derive(Debug)]
struct SpellError<'a> {
    line: &'a str,
    err_part: (usize, usize),
}

#[derive(Debug)]
struct SpellCheckedLine {
    line: String,
    err_parts: Vec<(usize, usize)>,
}

impl std::fmt::Display for SpellError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}~~{}~~{}",
            &self.line[..self.err_part.0],
            &self.line[self.err_part.0..self.err_part.1],
            &self.line[self.err_part.1..],
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

    fn spell_check_line<'a>(&self, line: String) -> SpellCheckedLine {
        SpellCheckedLine {
            err_parts: self
                .wordifier // #a
                .wordify(&line)
                .scan(0, |acc, word| {
                    let start = *acc;
                    *acc += word.len() + 1;
                    Some((start, word))
                })
                .filter_map(|(i, word)| {
                    // #a: To avoid redundancy, this method could be a default
                    // implementation on a trait (where `Self` implements the
                    // trait). We could directly do `self.spell_check`.
                    self.spell_checker
                        .spell_check(word)
                        .not()
                        .then_some((i, i + word.len()))
                })
                .collect(),
            line,
        }
    }

    fn run(self) {
        stdin()
            .lines()
            .enumerate()
            .filter_map(|(ln, line)| line.ok().map(|line| (ln + 1, self.spell_check_line(line))))
            .for_each(
                |(
                    ln,
                    SpellCheckedLine {
                        ref line,
                        err_parts,
                    },
                )| {
                    err_parts
                        .into_iter()
                        .map(|err_part| SpellError { line, err_part })
                        .for_each(|spell_err| println!("{ln}:{spell_err}"))
                },
            );
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
