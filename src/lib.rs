pub mod core;
pub mod dictionary;
pub mod error;
pub mod wordifier;

pub use core::Wizard;

#[cfg(test)]
mod test {
    use crate::{
        core::Wizard,
        dictionary::{BasicDictionary, Dictionary},
        wordifier::BasicWordifier,
    };

    fn small_dictionary() -> BasicDictionary {
        let mut d = BasicDictionary::default();

        d.learn("a");
        d.learn("an");
        d.learn("Bird");

        d
    }

    #[test]
    fn learned_spell() {
        let d = small_dictionary();

        assert!(d.contains("a"));
        assert!(d.contains("an"));
        assert!(d.contains("Bird"));
    }

    #[test]
    fn did_not_learn_misspelled() {
        assert!(!small_dictionary().contains("birdy"));
    }

    #[test]
    fn learned_spell_insensitively() {
        let d = small_dictionary();

        assert!(d.contains("A"));
        assert!(d.contains("aN"));
        assert!(d.contains("Bird"));
    }

    fn basic_wizard() -> Wizard<BasicDictionary, BasicWordifier> {
        Wizard {
            dictionary: small_dictionary(),
            wordifier: BasicWordifier,
        }
    }

    #[test]
    fn phrase() {
        assert!(basic_wizard().errors("a bird").next().is_none());
    }

    #[test]
    fn phrase_with_a_misspelled_word() {
        let wizard = basic_wizard();
        let mut errors = wizard.errors("a birdy");

        assert_eq!(errors.next().unwrap().typo(), "birdy");
        assert!(errors.next().is_none());
    }

    #[test]
    fn snake_cased_identifier() {
        let wizard = basic_wizard();

        assert!(wizard.errors("a_bird").next().is_none());
    }

    #[test]
    fn camel_cased_identifier() {
        let wizard = basic_wizard();

        assert!(wizard.errors("aBird").next().is_none());
    }
}
