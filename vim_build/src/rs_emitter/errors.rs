

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Type not found error: {0}")]
    TypeNotFound(String),
    #[error("Printer error: {0}")]
    PrinterError(#[from] super::super::printer::Error),
    #[error("Internal Error: {0}")]
    InternalError(String),
}

// Result is a type alias for handling errors.
pub type Result<T> = std::result::Result<T, Error>;