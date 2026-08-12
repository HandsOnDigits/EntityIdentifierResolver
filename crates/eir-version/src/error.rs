#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("EIR core error: {0}")]
    Core(#[from] eir_core::error::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("left database does not exist: {0}")]
    LeftDatabaseMissing(String),

    #[error("right database does not exist: {0}")]
    RightDatabaseMissing(String),

    #[error("merge output cannot be the same as an input database")]
    OutputInputCollision,

    #[error("merge output already exists: {0}")]
    OutputExists(String),

    #[error("output path has no parent directory")]
    InvalidOutputPath,

    #[error("invalid merge input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, Error>;
