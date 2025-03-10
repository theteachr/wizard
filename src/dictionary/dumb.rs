pub struct Dumb;

impl super::Dictionary for Dumb {
    fn contains(&self, word: &str) -> bool {
        return word.to_lowercase() != "world";
    }
}
