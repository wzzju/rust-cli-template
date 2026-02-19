//! # Model Module
//!
//! Contains domain models used throughout the application.

// region:    --- MatchLine

/// Represents a single line containing search matches.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchLine {
    /// The 1-based line number in the source input.
    pub line_number: usize,
    /// The full content of the line.
    pub line: String,
    /// A list of ranges indicating where matches occurred in the line.
    pub spans: Vec<core::ops::Range<usize>>,
}

// endregion: --- MatchLine
