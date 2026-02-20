//! # Handlers Module
//!
//! This module contains the implementation of command handlers.
//! Each handler corresponds to a specific subcommand or action.
//!
//! The handlers are responsible for:
//! - Validating domain-specific constraints
//! - Orchestrating calls to the `core` crate
//! - Formatting and returning results (or errors)
//!
//! Note: Handlers should generally remain ignorant of `clap` argument structures.
//! They should accept plain Rust types (strings, paths, bools) to ensure
//! decoupling between the CLI interface and the business logic.

// region:    --- Modules

pub mod search;

// endregion: --- Modules
