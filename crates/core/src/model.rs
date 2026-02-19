#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchLine {
    pub line_number: usize,
    pub line: String,
    pub spans: Vec<core::ops::Range<usize>>,
}
