pub mod core;
pub mod dictionary;
pub mod error;
pub mod wordifier;

pub use core::Wizard;

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{
        core::Wizard,
        dictionary::BasicDictionary,
        wordifier::{BasicWordifier, CamelCaseWordifier},
    };

    fn small_dictionary() -> BasicDictionary {
        BasicDictionary::new(["a", "an", "Bird"])
    }

    fn basic_wizard() -> Wizard<BasicDictionary, BasicWordifier> {
        Wizard {
            dictionary: small_dictionary(),
            wordifier: BasicWordifier,
        }
    }

    #[test]
    fn phrase_with_a_misspelled_word() {
        let wizard = basic_wizard();

        let errors = wizard
            .errors("a birdy")
            .map(|e| e.typo().to_owned())
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
            dictionary: small_dictionary(),
            wordifier: CamelCaseWordifier,
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
