pub trait Wordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str>;
}

pub struct SimpleWordifier;

struct SimpleWordifierIter<'a> {
    line: &'a str,
}

impl<'a> Iterator for SimpleWordifierIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let word_start = self.line.find(char::is_alphabetic)?;

        let word_end = self.line[word_start..]
            .find(|c: char| !c.is_alphabetic() && c != '\'')
            .map(|i| i + word_start)
            .unwrap_or(self.line.len());

        // Couldn't do the following.
        // ```
        // let (word, self.line) = &self.line.split_at(word_end);
        // ```
        // Weird error reported: Found a 3 tuple where a 2 tuple was expected.

        let word = &self.line[word_start..word_end];
        self.line = &self.line[word_end..];

        Some(word)
    }
}

impl Wordifier for SimpleWordifier {
    fn words<'a>(&self, line: &'a str) -> impl Iterator<Item = &'a str> {
        SimpleWordifierIter { line }
    }
}
