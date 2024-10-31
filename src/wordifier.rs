pub trait Wordifier {
    fn wordify<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str>;
}

pub struct SimpleWordifier;

impl Wordifier for SimpleWordifier {
    fn wordify<'a, 'b>(&'a self, line: &'b str) -> impl Iterator<Item = &'b str> {
        line.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
    }
}
