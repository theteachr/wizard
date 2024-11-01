pub trait Wordifier {
    fn wordify<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str>;
}

pub struct SimpleWordifier;

impl Wordifier for SimpleWordifier {
    fn wordify<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str> {
        line.split(|c: char| c != '\'' && (c.is_whitespace() || c.is_ascii_punctuation()))
            .filter(|word| !word.is_empty())
    }
}
