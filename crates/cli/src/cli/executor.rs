use std::io::Read;

use crate::{Result, handlers::search::search_and_render};

pub fn run(cmd: super::cmd::Cmd) -> Result<i32> {
    let mut input = String::new();
    match cmd.path {
        Some(path) => {
            input = std::fs::read_to_string(path)?;
        }
        None => {
            std::io::stdin().read_to_string(&mut input)?;
        }
    }

    let output = search_and_render(&input, &cmd.pattern, cmd.regex, cmd.ignore_case)?;
    print!("{output}");
    Ok(if output.is_empty() { 1 } else { 0 })
}
