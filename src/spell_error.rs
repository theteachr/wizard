#[derive(Debug)]
pub struct SpellError<'a> {
    line: &'a str,
    error: &'a str,
}

impl<'a> SpellError<'a> {
    pub fn new(line: &'a str, error: &'a str) -> Self {
        Self { line, error }
    }
}

impl std::fmt::Display for SpellError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start = self.error.as_ptr() as usize - self.line.as_ptr() as usize;
        write!(
            f,
            "{}~~{}~~{}",
            &self.line[..start],
            &self.error,
            &self.line[start + self.error.len()..],
        )
    }
}
