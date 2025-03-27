#[derive(PartialEq, Eq, Debug)]
pub struct Error<'a> {
    pub(crate) line: &'a str,
    pub(crate) word: &'a str,
}

impl std::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.word.as_ptr() as usize - self.line.as_ptr() as usize;
        write!(
            f,
            "{}~~{}~~{}",
            &self.line[..start],
            &self.word,
            &self.line[start + self.word.len()..],
        )
    }
}
