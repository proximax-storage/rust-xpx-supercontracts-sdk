//! Execution error statuses

/// Result of unsuccessful supercontract execution.
///
/// An execution error consists
/// of an error code and optional description.
/// Descriptions are mostly used for developer purposes,
/// not for interaction of the system with users.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExecutionError {
    /// User-defined error code.
    code: i8,
    /// Optional error description.
    description: Option<String>,
}

impl ExecutionError {
    /// Constructs a new `ExecutionError` instance with the given error code.
    pub fn new(code: i8) -> Self {
        Self {
            code,
            description: None,
        }
    }

    /// Constructs a new `ExecutionError` instance with the given error code and description.
    pub fn with_description<T: Into<String>>(code: i8, description: T) -> Self {
        Self {
            code,
            description: Some(description.into()),
        }
    }
}

impl From<Error> for ExecutionError {
    fn from(value: Error) -> ExecutionError {
        let description = format!("{}", value);
        ExecutionError::with_description(value as i8, description)
    }
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Save to storage function failed: AddFromBytes")]
    FunctionSaveToStorageAddFromBytes = -13,

    #[fail(display = "GetLocal failed: wrong index")]
    FunctionGetLocalWrongIndex = -14,
}
