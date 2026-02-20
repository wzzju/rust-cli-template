//! # Render Module
//!
//! Handles the rendering of search results, specifically applying color highlighting.

use crate::{Error, Result};
use owo_colors::OwoColorize;

// region:    --- Highlighting

/// Highlights matched spans in a line using ANSI color codes.
///
/// # Arguments
///
/// * `line` - The source line text.
/// * `spans` - A list of ranges to highlight.
///
/// # Returns
///
/// The string with ANSI escape codes applied to the specified spans.
///
/// # Errors
///
/// Returns `Error::InvalidSpan` if spans are out of bounds or overlapping in a way that breaks logic
/// (though current implementation handles basic non-overlapping valid spans).
pub fn highlight_line(line: &str, spans: &[core::ops::Range<usize>]) -> Result<String> {
    if spans.is_empty() {
        return Ok(line.to_string());
    }

    let mut output = String::new();
    let mut cursor = 0;

    for span in spans {
        if span.start > span.end || span.end > line.len() {
            return Err(Error::InvalidSpan);
        }

        if cursor < span.start {
            output.push_str(line.get(cursor..span.start).ok_or(Error::InvalidSpan)?);
        }

        let segment = line.get(span.start..span.end).ok_or(Error::InvalidSpan)?;
        output.push_str(&segment.yellow().bold().to_string());

        cursor = span.end;
    }

    if cursor < line.len() {
        output.push_str(line.get(cursor..).ok_or(Error::InvalidSpan)?);
    }

    Ok(output)
}

// endregion: --- Highlighting

// region:    --- Tests

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

// endregion: --- Tests
