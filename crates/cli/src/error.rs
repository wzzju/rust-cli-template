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
