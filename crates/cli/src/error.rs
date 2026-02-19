use derive_more::{Display, Error as DeriveError, From};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Display, DeriveError, From)]
pub enum Error {
    #[display("io error")]
    Io(std::io::Error),
    #[display("core error")]
    Core(rg_core::Error),
}
