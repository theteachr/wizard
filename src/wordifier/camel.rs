use super::iter::Iter;
use super::Wordifier;

/// Same as [`Alphabetic`](wizard::wordifier::Alphabetic), but treats uppercase letters as word boundaries.
///
/// # Usage
/// ```
/// use wizard::wordifier::{Wordifier, CamelCase};
///
/// let words = CamelCase.words("`camelCased` keys save space.").collect::<Vec<_>>();
/// assert_eq!(words, vec!["camel", "Cased", "keys", "save", "space"]);
/// ```
pub struct CamelCase;

// XXX: Combine `Wordifiers` by implementing some of the traits in `std::ops`?
// `CamelCase` would only need to deal with uppercase letters. A new `Wordifier`
// could be created by combining `Alphabetic` and `CamelCase`.

impl Wordifier for CamelCase {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str> {
        Iter {
            line,
            word_begin: char::is_alphabetic,
            word_end: |c| c.is_ascii_uppercase() || !c.is_alphabetic() && c != '\'',
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wordifier::{CamelCase, Wordifier};

    #[test]
    fn it_works() {
        assert_eq!(
            CamelCase.words("basicWordifier").collect::<Vec<_>>(),
            vec!["basic", "Wordifier"]
        )
    }
}
