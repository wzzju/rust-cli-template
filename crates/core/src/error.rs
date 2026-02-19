//! # Error Module
//!
//! Defines the custom `Error` type and `Result` alias for the core crate.
//! This follows the "no thiserror/anyhow" rule, using `derive_more` for boilerplate.

use derive_more::{Display, Error as DeriveError, From};

// region:    --- Result

/// Custom Result type for the core crate.
pub type Result<T> = core::result::Result<T, Error>;

// endregion: --- Result

// region:    --- Error

/// Core error type for the library.
#[derive(Debug, Display, DeriveError, From)]
pub enum Error {
    /// Returned when an empty pattern is provided.
    #[display("empty pattern")]
    EmptyPattern,

    /// Wraps regex compilation errors.
    #[display("regex error")]
    Regex(regex::Error),

    /// Returned when highlighting spans are invalid.
    #[display("invalid span")]
    InvalidSpan,

    /// Wraps a custom error message.
    ///
    /// This variant is used for ad-hoc errors that don't fit into other variants.
    /// It is similar to `anyhow::Error` but strictly typed within this enum.
    #[display("{}", _0)]
    #[error(ignore)]
    #[from(String, &String, &str)]
    Custom(String),
}

// region:    --- Error Boilerplate

impl Error {
    /// Creates a `Custom` error from any type that implements `std::error::Error`.
    ///
    /// This is useful for converting external errors that are not explicitly
    /// handled by other variants into a `Custom` error.
    pub fn custom_from_err(err: impl std::error::Error) -> Self {
        Self::Custom(err.to_string())
    }

    /// Creates a `Custom` error from a string-like value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rg_core::Error;
    ///
    /// let err = Error::custom("something went wrong");
    /// assert!(matches!(err, Error::Custom(_)));
    /// ```
    pub fn custom(val: impl Into<String>) -> Self {
        Self::Custom(val.into())
    }
}

// endregion: --- Error Boilerplate

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_error() {
        let err = Error::custom("something went wrong");
        assert!(matches!(err, Error::Custom(_)));
        assert_eq!(err.to_string(), "something went wrong");
    }

    #[test]
    fn test_custom_from_err() {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, "io error");
        let err = Error::custom_from_err(io_err);
        assert!(matches!(err, Error::Custom(_)));
        assert_eq!(err.to_string(), "io error");
    }

    #[test]
    fn test_from_string() {
        let err: Error = "string error".to_string().into();
        assert!(matches!(err, Error::Custom(_)));
        assert_eq!(err.to_string(), "string error");
    }

    #[test]
    fn test_from_str() {
        let err: Error = "str error".into();
        assert!(matches!(err, Error::Custom(_)));
        assert_eq!(err.to_string(), "str error");
    }
}

// endregion: --- Error
