#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    RegexError(regex::Error),
    DirectionParsingError(String),
    LineParsingError(String)
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<&regex::Error> for Error {
    fn from(error: &regex::Error) -> Self {
        Error::RegexError(error.clone())
    }
}