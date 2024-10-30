use std::io::stdin;

#[derive(Debug)]
struct SpellError<'a> {
    line: &'a str,
    bad_part: (usize, usize),
}

#[derive(Debug)]
struct BadLine {
    line: String,
    bad_parts: Vec<(usize, usize)>,
}

impl BadLine {
    fn iter_errors<'a>(&'a self) -> impl Iterator<Item = SpellError<'a>> {
        BadLineIter {
            line: &self.line,
            bad_parts: &self.bad_parts,
        }
    }
}

struct BadLineIter<'a> {
    line: &'a str,
    bad_parts: &'a [(usize, usize)],
}

impl<'a> Iterator for BadLineIter<'a> {
    type Item = SpellError<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(((start, end), rest)) = self.bad_parts.split_first() {
            self.bad_parts = rest;

            return Some(SpellError {
                line: &self.line,
                bad_part: (*start, *end),
            });
        }

        None
    }
}

impl std::fmt::Display for SpellError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start, end) = self.bad_part;

        write!(
            f,
            "{}~~{}~~{}",
            &self.line[..start],
            &self.line[start..end],
            &self.line[end..],
        )
    }
}

trait Wordifier {
    fn wordify<'a, 'b>(&'a self, line: &'b str) -> impl Iterator<Item = &'b str>;
}

struct Root<S: SpellChecker, W: Wordifier> {
    spell_checker: S,
    wordifer: W,
}

trait SpellChecker {
    fn spell_check(&self, word: &str) -> bool;
}

impl<S: SpellChecker, W: Wordifier> Root<S, W> {
    fn new(spell_checker: S, wordifer: W) -> Self {
        Self {
            spell_checker,
            wordifer,
        }
    }

    fn spell_check_line<'a>(&self, line: String) -> Option<BadLine> {
        let bad_parts = self
            .wordifer
            .wordify(&line)
            .scan(0, |acc, word| {
                let start = *acc;
                *acc += word.len() + 1;
                Some((start, word))
            })
            .filter_map(|(i, word)| {
                (!self.spell_checker.spell_check(word)).then_some((i, i + word.len()))
            })
            .collect();

        Some(BadLine { line, bad_parts })
    }

    fn run(self) {
        stdin()
            .lines()
            .enumerate()
            .filter_map(|(ln, line)| {
                line.ok().and_then(|line| {
                    self.spell_check_line(line)
                        .map(|bad_line| (ln + 1, bad_line))
                })
            })
            .for_each(|(ln, bad_line)| {
                bad_line
                    .iter_errors()
                    .for_each(|spell_err| println!("{ln}:{spell_err}"))
            });
    }
}

struct BadSpellChecker;
struct BadWordifer;

impl Wordifier for BadWordifer {
    fn wordify<'a, 'b>(&'a self, line: &'b str) -> impl Iterator<Item = &'b str> {
        line.split_whitespace()
    }
}

impl SpellChecker for BadSpellChecker {
    fn spell_check(&self, word: &str) -> bool {
        return word.to_lowercase() != "world";
    }
}

fn main() {
    Root::new(BadSpellChecker, BadWordifer).run()
}
