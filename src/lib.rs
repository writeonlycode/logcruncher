use std::{fs::File, io::BufReader};

use anyhow::Result;
use config::Config;
use serde_json::Value;

pub mod config;

pub fn run(config: Config) -> Result<()> {
    let file = File::open(&config.input)?;
    let reader = BufReader::new(file);
    let value: Value = serde_json::from_reader(reader)?;

    if config.count {
        match value {
            Value::Null => {}
            Value::Bool(v) => {}
            Value::Array(v) => {
                println!("{}", v.iter().count())
            }
            Value::Number(v) => {}
            Value::String(v) => {}
            Value::Object(v) => {}
        }
    }

    Ok(())
}
