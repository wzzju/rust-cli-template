//! # Search Module
//!
//! Implements the core search algorithms: literal string search and regex search.

use crate::{Error, MatchLine, Result};

// region:    --- Literal Search

/// Searches for a literal string pattern in the input.
///
/// # Arguments
///
/// * `input` - The multi-line string to search.
/// * `pattern` - The literal string to find.
///
/// # Returns
///
/// A vector of `MatchLine` structs containing found matches.
///
/// # Errors
///
/// Returns `Error::EmptyPattern` if the pattern is empty.
pub fn search_literal(input: &str, pattern: &str) -> Result<Vec<MatchLine>> {
    if pattern.is_empty() {
        return Err(Error::EmptyPattern);
    }

    let mut matches = Vec::new();
    for (index, line) in input.lines().enumerate() {
        if line.contains(pattern) {
            let mut spans = Vec::new();
            let mut offset = 0;
            while let Some(found) = line[offset..].find(pattern) {
                let start = offset + found;
                let end = start + pattern.len();
                spans.push(start..end);
                offset = end;
                if offset >= line.len() {
                    break;
                }
            }

            matches.push(MatchLine {
                line_number: index + 1,
                line: line.to_string(),
                spans,
            });
        }
    }

    Ok(matches)
}

// endregion: --- Literal Search

// region:    --- Regex Search

/// Searches for a regex pattern in the input.
///
/// # Arguments
///
/// * `input` - The multi-line string to search.
/// * `pattern` - The regex pattern to compile.
/// * `ignore_case` - Whether to enable case-insensitive matching.
///
/// # Returns
///
/// A vector of `MatchLine` structs containing found matches.
///
/// # Errors
///
/// Returns `Error::EmptyPattern` if the pattern is empty.
/// Returns `Error::Regex` if the pattern is invalid.
pub fn search_regex(input: &str, pattern: &str, ignore_case: bool) -> Result<Vec<MatchLine>> {
    if pattern.is_empty() {
        return Err(Error::EmptyPattern);
    }

    let full_pattern = if ignore_case {
        format!("(?i){pattern}")
    } else {
        pattern.to_string()
    };

    let regex = regex::Regex::new(&full_pattern).map_err(Error::Regex)?;
    let mut matches = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let mut spans = Vec::new();
        for m in regex.find_iter(line) {
            spans.push(m.start()..m.end());
        }
        if !spans.is_empty() {
            matches.push(MatchLine {
                line_number: index + 1,
                line: line.to_string(),
                spans,
            });
        }
    }

    Ok(matches)
}

// endregion: --- Regex Search

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn finds_literal_matches() -> Result<()> {
        let input = "alpha\nbeta\nalp";
        let matches = super::search_literal(input, "alp")?;
        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].line_number, 1);
        assert_eq!(matches[1].line_number, 3);
        Ok(())
    }

    #[test]
    fn finds_regex_matches_ignore_case() -> Result<()> {
        let input = "Alpha\nbeta\nALP";
        let matches = super::search_regex(input, "alp", true)?;
        assert_eq!(matches.len(), 2);
        Ok(())
    }
}

// endregion: --- Tests
