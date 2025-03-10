pub struct BasicWordifier;

struct Iter<'a> {
    line: &'a str,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let word_start = self.line.find(char::is_alphabetic)?;

        let word_end = self.line[word_start..]
            .find(|c: char| !c.is_alphabetic() && c != '\'')
            .map(|i| i + word_start)
            .unwrap_or(self.line.len());

        let word = &self.line[word_start..word_end];

        self.line = &self.line[word_end..];

        Some(word)
    }
}

impl super::Wordifier for BasicWordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str> {
        Iter { line }
    }
}
