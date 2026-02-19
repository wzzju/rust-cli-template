mod error;
mod model;
mod render;
mod search;

pub use error::{Error, Result};
pub use model::MatchLine;
pub use render::highlight_line;
pub use search::{search_literal, search_regex};

pub fn ping() -> &'static str {
    "ok"
}

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn smoke_compiles() -> Result<()> {
        Ok(())
    }
}
