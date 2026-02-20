//! # Executor Module
//!
//! Handles the execution flow of the CLI command.

use std::io::Read;

use crate::{Result, handlers::search::search_and_render};
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
    if cmd.verbose {
        eprintln!("DEBUG: Verbose mode enabled");
        eprintln!("DEBUG: Command structure: {:#?}", cmd);
    }

    match cmd.subcmd {
        Some(super::cmd::SubCommand::Search(args)) => run_search(args, cmd.verbose),
        None => {
            super::cmd::Cmd::command().print_help()?;
            Ok(1)
        }
    }
}

fn run_search(args: super::cmd::SearchArgs, verbose: bool) -> Result<i32> {
    let mut input = String::new();

    // Determine input source: file or stdin.
    match args.path {
        Some(ref path) => {
            if verbose {
                eprintln!("DEBUG: Reading from file: {:?}", path);
            }
            input = std::fs::read_to_string(path)?;
        }
        None => {
            if verbose {
                eprintln!("DEBUG: Reading from stdin");
            }
            std::io::stdin().read_to_string(&mut input)?;
        }
    }

    if verbose {
        eprintln!("DEBUG: Searching for pattern: {:?}", args.pattern);
        eprintln!("DEBUG: Case insensitive: {}", args.ignore_case);
        eprintln!("DEBUG: Regex mode: {}", args.regex);
    }

    // Execute search and rendering.
    let output = search_and_render(&input, &args.pattern, args.regex, args.ignore_case)?;

    // Output result.
    print!("{output}");

    // Return appropriate exit code.
    Ok(if output.is_empty() { 1 } else { 0 })
}

// endregion: --- Run
