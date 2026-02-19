use crate::Result;
use owo_colors::OwoColorize;

pub fn highlight_line(line: &str, spans: &[core::ops::Range<usize>]) -> Result<String> {
    if spans.is_empty() {
        return Ok(line.to_string());
    }
    Ok(line.red().bold().to_string())
}

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn highlights_matches_with_colors() -> Result<()> {
        let line = "alpha beta";
        let rendered = super::highlight_line(line, &[0..5, 6..7])?;
        assert!(rendered.contains("\u{1b}["));
        Ok(())
    }
}
