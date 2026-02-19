//! # Command Module
//!
//! Defines the command-line arguments and flags using `clap`.

use clap::{Args, Parser, Subcommand};

// region:    --- Cmd

/// The main command structure for the CLI.
#[derive(Debug, Parser)]
#[command(name = "rg", version, about = "A simple grep-like tool")]
pub struct Cmd {
    #[command(subcommand)]
    pub subcmd: Option<SubCommand>,
}

// endregion: --- Cmd

// region:    --- SubCommand

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Search for a pattern in a file.
    Search(SearchArgs),
}

// endregion: --- SubCommand

// region:    --- Args

#[derive(Debug, Args)]
pub struct SearchArgs {
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

// endregion: --- Args
