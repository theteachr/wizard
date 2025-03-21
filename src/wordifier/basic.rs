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

#[cfg(test)]
mod tests {
    use crate::wordifier::{BasicWordifier, Wordifier};

    #[test]
    fn it_works() {
        assert_eq!(
            BasicWordifier.words("this is good").collect::<Vec<_>>(),
            vec!["this", "is", "good"]
        )
    }

    #[test]
    fn with_punctuation() {
        assert_eq!(
            BasicWordifier.words("Hello, world!").collect::<Vec<_>>(),
            vec!["Hello", "world"]
        )
    }

    #[test]
    fn with_underscores() {
        assert_eq!(
            BasicWordifier.words("basic_wordifier").collect::<Vec<_>>(),
            vec!["basic", "wordifier"]
        )
    }

    #[test]
    fn does_not_split_camel_case() {
        assert_ne!(
            BasicWordifier.words("basicWordifier").collect::<Vec<_>>(),
            vec!["basic", "Wordifier"]
        )
    }
}
