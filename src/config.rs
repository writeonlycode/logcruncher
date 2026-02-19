use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(about)]
pub struct Config {
    pub input: String,

    #[arg(long, help = "Filter records (repeatable)")]
    pub filter: Option<Vec<String>>,

    #[arg(long, help = "Group records by field")]
    pub group_by: Option<String>,

    #[arg(long, help = "Count records")]
    pub count: bool,

    #[arg(long, default_value = "json", help = "Output format")]
    pub format: Option<Format>,

    #[arg(long, help = "Limit number of output rows")]
    pub limit: Option<usize>,

    #[arg(long, help = "Fail on malformed input")]
    pub strict: bool,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Format {
    Json,
    Table,
}
