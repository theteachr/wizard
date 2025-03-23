/// A dumb dictionary according to which "world" is the only incorrectly spelt
/// word.
pub struct Dumb;

impl super::Dictionary for Dumb {
    fn contains(&self, word: &str) -> bool {
        return word.to_lowercase() != "world";
    }
}
