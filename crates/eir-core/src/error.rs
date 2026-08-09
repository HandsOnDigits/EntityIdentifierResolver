use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serialization(String),
    EntityAlreadyExists(String),
    EntityNotFound(usize),
    InvalidFormat(String),
    UnsupportedVersion(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "IO error: {error}"),
            Self::Serialization(error) => {
                write!(f, "Serialization error: {error}")
            }
            Self::EntityAlreadyExists(id) => {
                write!(f, "Entity already exists: {id}")
            }
            Self::EntityNotFound(id) => {
                write!(f, "Entity not found: {id}")
            }

            Self::InvalidFormat(e) => write!(f, "invalid EIR format: {e}"),
            Self::UnsupportedVersion(version) => {
                write!(f, "unsupported EIR format version: {version}")
            }
        }
    }
}

impl StdError for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
