//! # Executor Module
//!
//! Handles the execution flow of the CLI command.

use std::io::Read;

use crate::{handlers::search::search_and_render, Result};
use clap::CommandFactory;

// region:    --- Run

/// Executes the main logic based on the parsed command.
///
/// # Arguments
///
/// * `cmd` - The parsed command arguments.
///
/// # Returns
///
/// An integer exit code (0 for success/matches found, 1 for no matches or error).
pub fn run(cmd: super::cmd::Cmd) -> Result<i32> {
    match cmd.subcmd {
        Some(super::cmd::SubCommand::Search(args)) => run_search(args),
        None => {
            super::cmd::Cmd::command().print_help()?;
            Ok(1)
        }
    }
}

fn run_search(args: super::cmd::SearchArgs) -> Result<i32> {
    let mut input = String::new();

    // Determine input source: file or stdin.
    match args.path {
        Some(path) => {
            input = std::fs::read_to_string(path)?;
        }
        None => {
            std::io::stdin().read_to_string(&mut input)?;
        }
    }

    // Execute search and rendering.
    let output = search_and_render(&input, &args.pattern, args.regex, args.ignore_case)?;

    // Output result.
    print!("{output}");

    // Return appropriate exit code.
    Ok(if output.is_empty() { 1 } else { 0 })
}

// endregion: --- Run
