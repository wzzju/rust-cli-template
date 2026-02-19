mod cli;
mod error;
mod handlers;

use clap::Parser;
pub use error::{Error, Result};

fn main() {
    let cmd = cli::cmd::Cmd::parse();
    match cli::executor::run(cmd) {
        Ok(code) => std::process::exit(code),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
