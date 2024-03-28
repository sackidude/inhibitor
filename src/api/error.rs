#[derive(Debug)]
pub enum Error {
    ParseError,
    VerificationError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "internal api error")
    }
}

impl std::error::Error for Error {}
