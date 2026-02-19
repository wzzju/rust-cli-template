use crate::Result;

pub fn search_and_render(
    input: &str,
    pattern: &str,
    use_regex: bool,
    ignore_case: bool,
) -> Result<String> {
    let matches = if use_regex {
        rg_core::search_regex(input, pattern, ignore_case)?
    } else {
        rg_core::search_literal(input, pattern)?
    };

    let mut output = String::new();
    for m in matches {
        let line = rg_core::highlight_line(&m.line, &m.spans)?;
        output.push_str(&format!("{}:{line}\n", m.line_number));
    }

    Ok(output)
}
