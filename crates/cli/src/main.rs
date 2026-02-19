//! # CLI Crate
//!
//! The command-line interface entry point for the `rg` tool.
//! Handles argument parsing, error reporting, and command dispatch.

// region:    --- Modules

mod cli;
mod error;
mod handlers;

// endregion: --- Modules

// region:    --- Imports

use clap::Parser;
pub use error::{Error, Result};

// endregion: --- Imports

// region:    --- Main

fn main() {
    let cmd = cli::cmd::Cmd::parse();

    // Run the command and handle any errors.
    match cli::executor::run(cmd) {
        Ok(code) => std::process::exit(code),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}

// endregion: --- Main
