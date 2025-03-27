pub mod core;
pub mod error;
pub mod lexicon;
pub mod wordifier;

pub use core::Wizard;

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{
        core::Wizard,
        error::Error,
        lexicon::Set,
        wordifier::{Alphabetic, CamelCase},
    };

    fn lexicon() -> Set {
        Set::new(["a", "an", "Bird"])
    }

    fn basic_wizard() -> Wizard<Set, Alphabetic> {
        Wizard {
            lexicon: lexicon(),
            wordifier: Alphabetic,
        }
    }

    #[test]
    fn phrase_with_a_misspelled_word() {
        let wizard = basic_wizard();

        let errors = wizard
            .errors("a birdy")
            .map(|Error { word, .. }| word.to_owned())
            .collect::<Vec<_>>();

        assert_eq!(errors, vec!["birdy"]);
    }

    #[test]
    fn snake_cased_identifier() {
        let wizard = basic_wizard();

        assert_eq!(wizard.errors("a_bird").collect::<Vec<_>>(), vec![]);
    }

    #[test]
    fn camel_cased_identifier() {
        let wizard = Wizard {
            lexicon: lexicon(),
            wordifier: CamelCase,
        };

        assert!(wizard.errors(r#"aBird = "bird""#).next().is_none());

        assert_eq!(
            wizard
                .errors(r#"aBirb = "birb""#)
                .map(|e| e.to_string())
                .collect::<Vec<_>>(),
            vec![r#"a~~Birb~~ = "birb""#, r#"aBirb = "~~birb~~""#]
        );
    }
}
