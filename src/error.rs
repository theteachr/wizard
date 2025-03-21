#[derive(PartialEq, Eq, Debug)]
pub struct Error<'a> {
    line: &'a str,
    error: &'a str,
}

impl<'a> Error<'a> {
    pub(crate) fn new(line: &'a str, error: &'a str) -> Self {
        Self { line, error }
    }

    pub fn typo(&self) -> &str {
        self.error
    }
}

impl std::fmt::Display for Error<'_> {
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
