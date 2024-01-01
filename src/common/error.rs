use failure::Fail;
use std::{io, string};

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

    /// Sled error
    #[fail(display = "{}", _0)]
    Sled(#[cause] sled::Error),

    /// FromUtf8 Error
    #[fail(display = "Utf8 Error {}", _0)]
    Utf8Error(#[cause] string::FromUtf8Error),

    /// Build ThreadPool Error
    #[fail(display = "Build ThreadPool Error {}", _0)]
    ThreadPoolBuildError(#[cause] rayon::ThreadPoolBuildError),

    /// Key not found error
    #[fail(display = "Key not found")]
    KeyNotFound,

    /// Unknown command type error
    #[fail(display = "Unknown command type")]
    UnknownCommandType,

    /// Unknown engine type error
    #[fail(display = "Unknown engine type")]
    UnknownEngineType,

    /// Unknown engine type error
    #[fail(display = "Change engine after initialization")]
    ChangeEngineError,

    /// common string error
    #[fail(display = "{}", _0)]
    CommonStringError(String),
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

/// Implements the conversion from `sled::Error` to `KVStoreError`.
impl From<sled::Error> for KVStoreError {
    /// Converts a `sled::Error` into a `KVStoreError`.
    ///
    /// # Arguments
    ///
    /// * `err` - The `sled::Error` to convert.
    ///
    /// # Returns
    ///
    /// The converted `KVStoreError`.
    fn from(err: sled::Error) -> Self {
        KVStoreError::Sled(err)
    }
}

/// Implements the conversion from `std::string::FromUtf8Error` to `KVStoreError`.
impl From<string::FromUtf8Error> for KVStoreError {
    /// Converts a `std::string::FromUtf8Error` into a `KVStoreError::Utf8Error`.
    ///
    /// # Arguments
    ///
    /// * `err` - The `std::string::FromUtf8Error` to convert.
    ///
    /// # Returns
    ///
    /// The converted `KVStoreError::Utf8Error`.
    fn from(err: string::FromUtf8Error) -> Self {
        KVStoreError::Utf8Error(err)
    }
}

/// Implements the conversion from `rayon::ThreadPoolBuildError` to `KVStoreError`.
impl From<rayon::ThreadPoolBuildError> for KVStoreError {
    /// Converts the `rayon::ThreadPoolBuildError` into `KVStoreError`.
    ///
    /// # Arguments
    ///
    /// * `err` - The `rayon::ThreadPoolBuildError` to be converted.
    ///
    /// # Returns
    ///
    /// The converted `KVStoreError` instance.
    fn from(err: rayon::ThreadPoolBuildError) -> Self {
        KVStoreError::ThreadPoolBuildError(err)
    }
}
