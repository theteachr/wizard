use super::iter::Iter;

pub struct BasicWordifier;

impl super::Wordifier for BasicWordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str> {
        Iter {
            line,
            word_begin: char::is_alphabetic,
            word_end: |c: char| !c.is_alphabetic() && c != '\'',
        }
    }
}
