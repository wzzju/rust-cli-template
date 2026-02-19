//! # Command Module
//!
//! Defines the command-line arguments and flags using `clap`.

use clap::Parser;

// region:    --- Cmd

/// The main command structure for the CLI.
#[derive(Debug, Parser)]
pub struct Cmd {
    /// The pattern to search for.
    pub pattern: String,

    /// Case-insensitive search.
    #[arg(short, long)]
    pub ignore_case: bool,

    /// Treat the pattern as a regular expression.
    #[arg(short, long)]
    pub regex: bool,

    /// The file path to search (optional).
    /// If not provided, reads from stdin.
    pub path: Option<std::path::PathBuf>,
}

// endregion: --- Cmd
