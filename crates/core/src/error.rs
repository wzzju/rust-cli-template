//! # Error Module
//!
//! Defines the custom `Error` type and `Result` alias for the core crate.
//! This follows the "no thiserror/anyhow" rule, using `derive_more` for boilerplate.

use derive_more::{Display, Error as DeriveError};

// region:    --- Result

/// Custom Result type for the core crate.
pub type Result<T> = core::result::Result<T, Error>;

// endregion: --- Result

// region:    --- Error

/// Core error type for the library.
#[derive(Debug, Display, DeriveError)]
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
}

// endregion: --- Error
