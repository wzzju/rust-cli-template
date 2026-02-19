//! # Error Module
//!
//! Defines the `Error` type for the CLI crate, unifying errors from I/O and the core crate.

use derive_more::{Display, Error as DeriveError, From};

// region:    --- Result

/// Custom Result type for the CLI crate.
pub type Result<T> = core::result::Result<T, Error>;

// endregion: --- Result

// region:    --- Error

/// Top-level error type for the CLI.
///
/// Note on `From` derivation:
/// Since all variants are single-field tuple variants (`Io(std::io::Error)`, `Core(rg_core::Error)`),
/// `derive_more::From` automatically generates `From<FieldType>` for each variant.
/// No explicit `#[from]` attribute is needed here.
#[derive(Debug, Display, DeriveError, From)]
pub enum Error {
    /// Standard I/O errors.
    #[display("io error")]
    Io(std::io::Error),

    /// Errors propagated from the core crate.
    #[display("core error")]
    Core(rg_core::Error),
}

// endregion: --- Error

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::other("io error");
        let err: Error = io_err.into();
        assert!(matches!(err, Error::Io(_)));
        assert_eq!(err.to_string(), "io error");
    }

    #[test]
    fn test_from_core_error() {
        let core_err = rg_core::Error::EmptyPattern;
        let err: Error = core_err.into();
        assert!(matches!(err, Error::Core(_)));
        assert_eq!(err.to_string(), "core error");
    }
}

// endregion: --- Tests
