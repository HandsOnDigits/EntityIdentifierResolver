use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl StdError for Error {}

pub type Result<T> = std::result::Result<T, Error>;
