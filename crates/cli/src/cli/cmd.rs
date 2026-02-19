use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cmd {
    pub pattern: String,
    #[arg(short, long)]
    pub ignore_case: bool,
    #[arg(short, long)]
    pub regex: bool,
    pub path: Option<std::path::PathBuf>,
}
