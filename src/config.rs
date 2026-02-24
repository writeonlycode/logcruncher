use std::str::FromStr;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(about)]
pub struct Config {
    pub input: String,

    #[arg(long, help = "Filter records (repeatable)")]
    pub filter: Option<Vec<Filter>>,

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

#[derive(Clone, Debug)]
pub struct Filter {
    pub field: String,
    pub value: String,
}

impl FromStr for Filter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field, value) = s
            .split_once("=")
            .ok_or("filter must be in the form field=value")?;

        if field.is_empty() || value.is_empty() {
            return Err("filter and value must not be empty".to_string());
        }

        Ok(Filter {
            field: field.to_string(),
            value: value.to_string(),
        })
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Format {
    Json,
    Table,
}
