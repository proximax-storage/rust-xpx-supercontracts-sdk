//! Execution error statuses

use std::result;
pub type Result<T> = result::Result<T, self::Error>;
pub type FunctionResult = Result<i64>;
pub type MultipleFunctionResult = Result<Vec<u8>>;

/// Success status of function execution
/// when function don't has own result.
pub const STATUS_SUCCESS: i64 = 0;

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

#[repr(i8)]
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed serialize the given data structure as a JSON byte ")]
    SerializeJson = -1,
    #[fail(display = "Failed deserialize the given byte data to structure")]
    DeserializeJson = -2,
}
