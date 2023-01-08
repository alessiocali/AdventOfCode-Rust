#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    Parsing(ParsingError)
}

#[derive(Debug)]
pub enum ParsingError {
    InvalidTreeHeight(char)
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<ParsingError> for Error {
    fn from(error: ParsingError) -> Self {
        Error::Parsing(error)
    }
}