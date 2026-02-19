//! # Core Crate
//!
//! This crate contains the core domain logic for the `rg` CLI tool.
//! It includes search algorithms, domain models, and rendering logic.
//!
//! # Examples
//!
//! ```
//! use rg_core::{search_literal, MatchLine};
//!
//! let input = "hello\nworld";
//! let matches = search_literal(input, "hello").unwrap();
//! assert_eq!(matches.len(), 1);
//! ```

// region:    --- Modules

mod error;
mod model;
mod render;
mod search;

// endregion: --- Modules

// region:    --- Exports

pub use error::{Error, Result};
pub use model::MatchLine;
pub use render::highlight_line;
pub use search::{search_literal, search_regex};

// endregion: --- Exports

/// Simple ping function to verify crate is working.
pub fn ping() -> &'static str {
    "ok"
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn smoke_compiles() -> Result<()> {
        Ok(())
    }
}

// endregion: --- Tests
