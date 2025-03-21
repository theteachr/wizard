pub struct Iter<'a> {
    pub line: &'a str,
    pub word_begin: fn(char) -> bool,
    pub word_end: fn(char) -> bool,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let word_start = self.line.find(self.word_begin)?;

        let word_end = self.line[word_start + 1..]
            .find(self.word_end)
            .map(|i| i + word_start + 1)
            .unwrap_or(self.line.len());

        let word = &self.line[word_start..word_end];

        self.line = &self.line[word_end..];

        Some(word)
    }
}
