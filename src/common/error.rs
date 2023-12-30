use failure::Fail;
use std::io;

/// well-defined Result
pub type Result<T> = std::result::Result<T, KVStoreError>;

#[derive(Fail, Debug)]
/// well-defined Error
pub enum KVStoreError {
    /// Io error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Serde error
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    /// Key not found error
    #[fail(display = "Key not found")]
    KeyNotFound,

    /// Unknown command type error
    #[fail(display = "Unknown command type")]
    UnknownCommandType,
}

/// Implements the conversion from `io::Error` to `KVStoreError`.
impl From<io::Error> for KVStoreError {
    /// Converts an `io::Error` into a `KVStoreError`.
    ///
    /// # Arguments
    ///
    /// * `err` - The `io::Error` to convert.
    ///
    /// # Returns
    ///
    /// The converted `KVStoreError`.
    fn from(err: io::Error) -> Self {
        KVStoreError::Io(err)
    }
}

/// Implements the conversion from `serde_json::Error` to `KVStoreError`.
impl From<serde_json::Error> for KVStoreError {
    /// Converts a `serde_json::Error` into a `KVStoreError`.
    ///
    /// # Arguments
    ///
    /// * `err` - The `serde_json::Error` to convert.
    ///
    /// # Returns
    ///
    /// The converted `KVStoreError`.
    fn from(err: serde_json::Error) -> Self {
        KVStoreError::Serde(err)
    }
}