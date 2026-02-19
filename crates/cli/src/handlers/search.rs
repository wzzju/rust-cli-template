//! # Search Handler
//!
//! orchestrates the search logic by calling the core crate and rendering the results.

use crate::Result;

// region:    --- Search & Render

/// Performs the search and returns the rendered output.
///
/// # Arguments
///
/// * `input` - The content to search.
/// * `pattern` - The pattern to search for.
/// * `use_regex` - Whether to use regex search.
/// * `ignore_case` - Whether to ignore case.
///
/// # Returns
///
/// The formatted output string with line numbers and highlighting.
pub fn search_and_render(
    input: &str,
    pattern: &str,
    use_regex: bool,
    ignore_case: bool,
) -> Result<String> {
    // Dispatch to appropriate core search function.
    let matches = if use_regex {
        rg_core::search_regex(input, pattern, ignore_case)?
    } else {
        rg_core::search_literal(input, pattern)?
    };

    // Render results.
    let mut output = String::new();
    for m in matches {
        let line = rg_core::highlight_line(&m.line, &m.spans)?;
        output.push_str(&format!("{}:{line}\n", m.line_number));
    }

    Ok(output)
}

// endregion: --- Search & Render
