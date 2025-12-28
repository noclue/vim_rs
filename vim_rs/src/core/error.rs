//! Unified error type for PropertyCollector and Task operations.
//!
//! This module provides a single error type for all PropertyCollector and task-related
//! operations, using the ErrorKind + ErrorSource pattern for stability and extensibility.
//!
//! # Pattern Matching on Errors
//!
//! Use the [`Error::kind()`] method for stable pattern matching:
//!
//! ```ignore
//! use vim_rs::core::error::ErrorKind;
//!
//! match result {
//!     Ok(value) => println!("Success: {:?}", value),
//!     Err(err) => match err.kind() {
//!         ErrorKind::TaskCancelled => println!("Task was cancelled"),
//!         ErrorKind::TaskFailed => {
//!             // Access the underlying MethodFault
//!             if let Some(fault) = err.task_fault() {
//!                 println!("Task failed with fault: {:?}", fault);
//!             }
//!         }
//!         ErrorKind::RemoteCommunication => {
//!             // Access the underlying client::Error
//!             if let Some(client_err) = err.client_error() {
//!                 println!("Client error: {}", client_err);
//!             }
//!         }
//!         _ => println!("Other error: {}", err),
//!     }
//! }
//! ```

use std::fmt;
use thiserror::Error;
use crate::types::structs::MethodFault;
use super::client;

/// Result type alias for PropertyCollector and Task operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Stable error category for pattern matching.
///
/// This enum is marked `#[non_exhaustive]`, which means adding new variants is a
/// non-breaking change. Users should always include a wildcard arm in match statements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// HTTP or network transport error (wraps `client::Error`).
    RemoteCommunication,
    /// Invalid data type returned for a property.
    InvalidPropertyType,
    /// A required field was None.
    MissingRequiredField,
    /// Unexpected property path encountered.
    UnexpectedPropertyPath,
    /// No data found in ObjectUpdate/ObjectContent.
    NoDataFound,
    /// Task was cancelled.
    TaskCancelled,
    /// Task completed with an error/fault.
    TaskFailed,
    /// Lock poisoned (typically from RwLock/Mutex).
    LockPoisoned,
    /// Internal error or unexpected condition.
    Internal,
}

/// Internal representation of error payloads.
///
/// This enum is NOT part of the public API. Users cannot match on these variants directly,
/// only on the stable `ErrorKind` categories.
#[derive(Debug, Error)]
enum ErrorSource {
    #[error("invalid property type for '{property}': expected {expected}, got {got}")]
    InvalidPropertyType {
        property: String,
        expected: String,
        got: String,
    },

    #[error("required field '{0}' was None")]
    MissingRequiredField(String),

    #[error("no data found in ObjectUpdate/ObjectContent")]
    NoDataFound,

    #[error("unexpected property path: {0}")]
    UnexpectedPropertyPath(String),

    #[error("remote communication error: {0:?}")]
    RemoteCommunication(#[from] client::Error),

    #[error("task cancelled")]
    TaskCancelled,

    #[error("task failed: {0:?}")]
    TaskFailed(MethodFault),

    #[error("lock poisoned: {0}")]
    LockPoisoned(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// The unified error type for PropertyCollector and Task operations.
///
/// This struct exposes a stable `ErrorKind` for pattern matching while keeping
/// the internal error representation private and evolvable.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    source: ErrorSource,
}

impl Error {
    /// Returns the stable category of this error.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use vim_rs::core::error::ErrorKind;
    ///
    /// match err.kind() {
    ///     ErrorKind::TaskCancelled => println!("Task was cancelled"),
    ///     ErrorKind::RemoteCommunication => println!("Network error"),
    ///     _ => println!("Other error: {}", err),
    /// }
    /// ```
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Returns a reference to the underlying `MethodFault` if this is a `TaskFailed` error.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if let Some(fault) = err.task_fault() {
    ///     println!("Task failed with fault type: {:?}", fault.type_);
    /// }
    /// ```
    pub fn task_fault(&self) -> Option<&MethodFault> {
        match &self.source {
            ErrorSource::TaskFailed(fault) => Some(fault),
            _ => None,
        }
    }

    /// Returns a reference to the underlying `client::Error` if this is a `RemoteCommunication` error.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use vim_rs::core::error::ErrorKind;
    ///
    /// if err.kind() == ErrorKind::RemoteCommunication {
    ///     if let Some(client_err) = err.client_error() {
    ///         // Handle client error specifically
    ///         eprintln!("Client error: {}", client_err);
    ///     }
    /// }
    /// ```
    pub fn client_error(&self) -> Option<&client::Error> {
        match &self.source {
            ErrorSource::RemoteCommunication(e) => Some(e),
            _ => None,
        }
    }

    // Private constructor
    fn new(kind: ErrorKind, source: ErrorSource) -> Self {
        Self { kind, source }
    }

    /// Create an error for an invalid property type.
    pub fn invalid_property_type(property: String, expected: String, got: String) -> Self {
        Self::new(
            ErrorKind::InvalidPropertyType,
            ErrorSource::InvalidPropertyType {
                property,
                expected,
                got,
            },
        )
    }

    /// Create an error for a missing required field.
    pub fn missing_required_field(field: String) -> Self {
        Self::new(
            ErrorKind::MissingRequiredField,
            ErrorSource::MissingRequiredField(field),
        )
    }

    /// Create an error for no data found.
    pub fn no_data_found() -> Self {
        Self::new(ErrorKind::NoDataFound, ErrorSource::NoDataFound)
    }

    /// Create an error for an unexpected property path.
    pub fn unexpected_property_path(path: String) -> Self {
        Self::new(
            ErrorKind::UnexpectedPropertyPath,
            ErrorSource::UnexpectedPropertyPath(path),
        )
    }

    /// Create an error for a task cancellation.
    pub fn task_cancelled() -> Self {
        Self::new(ErrorKind::TaskCancelled, ErrorSource::TaskCancelled)
    }

    /// Create an error for a task failure with a method fault.
    pub fn task_failed(fault: MethodFault) -> Self {
        Self::new(ErrorKind::TaskFailed, ErrorSource::TaskFailed(fault))
    }

    /// Create an error for a poisoned lock.
    pub fn lock_poisoned(msg: String) -> Self {
        Self::new(ErrorKind::LockPoisoned, ErrorSource::LockPoisoned(msg))
    }

    /// Create an internal error with a message.
    pub fn internal(msg: String) -> Self {
        Self::new(ErrorKind::Internal, ErrorSource::Internal(msg))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.source()
    }
}

// Conversion implementations

impl From<client::Error> for Error {
    fn from(e: client::Error) -> Self {
        Error::new(ErrorKind::RemoteCommunication, ErrorSource::RemoteCommunication(e))
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for Error {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error::new(ErrorKind::Internal, ErrorSource::Other(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(ErrorKind::Internal, ErrorSource::Other(Box::new(e)))
    }
}



