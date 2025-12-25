use std::fmt;

use crate::types::structs::MethodFault;

/// Errors returned by [`crate::core::tasks::TaskTracker`].
#[derive(Debug)]
pub enum TaskError {
    /// A vSphere/API/client-side error (SOAP/REST call, property collector, etc.)
    Vim(crate::core::pc_helpers::Error),
    /// Client-level error (HTTP/serde/method fault) from `core::client`.
    Client(crate::core::client::Error),
    /// JSON serialization/deserialization error (used for cloning/decoding task results).
    Serde(serde_json::Error),
    /// The task was cancelled.
    Cancelled,
    /// The task completed with an error/fault payload.
    TaskFailed(MethodFault),
    /// Any other error (usually internal / unexpected).
    Other(String),
}

impl TaskError {
    /// Create a generic error with a message (used for unexpected/internal failures).
    pub fn new_other(msg: String) -> Self {
        Self::Other(msg)
    }

    /// Create a cancellation error.
    pub fn new_cancelled() -> Self {
        Self::Cancelled
    }

    /// Wrap a vSphere `MethodFault` as a task failure.
    pub fn new_task_error(fault: MethodFault) -> Self {
        Self::TaskFailed(fault)
    }
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::Vim(e) => write!(f, "vSphere error: {e}"),
            TaskError::Client(e) => write!(f, "client error: {e}"),
            TaskError::Serde(e) => write!(f, "serde_json error: {e}"),
            TaskError::Cancelled => write!(f, "task cancelled"),
            TaskError::TaskFailed(v) => write!(f, "task failed: {v:?}"),
            TaskError::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for TaskError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TaskError::Vim(e) => Some(e),
            TaskError::Client(e) => Some(e),
            TaskError::Serde(e) => Some(e),
            TaskError::Cancelled | TaskError::TaskFailed(_) | TaskError::Other(_) => None,
        }
    }
}

impl From<crate::core::pc_helpers::Error> for TaskError {
    fn from(value: crate::core::pc_helpers::Error) -> Self {
        Self::Vim(value)
    }
}

impl From<crate::core::client::Error> for TaskError {
    fn from(value: crate::core::client::Error) -> Self {
        Self::Client(value)
    }
}

impl From<serde_json::Error> for TaskError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}


