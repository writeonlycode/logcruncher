use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(about)]
pub struct Config {
    input: String,

    #[arg(long, help = "Filter records (repeatable)")]
    filter: Option<Vec<String>>,

    #[arg(long, help = "Group records by field")]
    group_by: Option<String>,

    #[arg(long, help = "Count records")]
    count: bool,

    #[arg(long, default_value = "json", help = "Output format")]
    format: Option<Format>,

    #[arg(long, help = "Limit number of output rows")]
    limit: Option<usize>,

    #[arg(long, help = "Fail on malformed input")]
    strict: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Json,
    Table,
}
