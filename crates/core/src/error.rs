use derive_more::{Display, Error as DeriveError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, DeriveError)]
pub enum Error {
    #[display("empty pattern")]
    EmptyPattern,
    #[display("regex error")]
    Regex(regex::Error),
    #[display("invalid span")]
    InvalidSpan,
}
