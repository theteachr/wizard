use super::iter::Iter;
use super::Wordifier;

pub struct CamelCaseWordifier;

impl Wordifier for CamelCaseWordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str> {
        Iter {
            line,
            word_begin: char::is_alphabetic,
            word_end: |c: char| c.is_ascii_uppercase() || !c.is_alphabetic() && c != '\'',
        }
    }
}
