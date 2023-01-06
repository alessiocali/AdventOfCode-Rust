#[derive(Debug)]
pub enum ParsingError { 
    InvalidFileSize,
    InvalidLine(String),
    NoCurrentDirectory,
    NoParentDirectory,
    NoRootDirectory,
    UnrecognizedSyntax(String)
}

#[derive(Debug)]
pub enum Error { 
    IoError(std::io::Error), 
    ParsingError(ParsingError), 
    RegexError(regex::Error)
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<ParsingError> for Error {
    fn from(error: ParsingError) -> Self {
        Error::ParsingError(error)
    }
}

impl From<&regex::Error> for Error {
    fn from(error: &regex::Error) -> Self {
        Error::RegexError(error.clone())
    }
}