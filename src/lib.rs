use std::{fs::File, io::BufReader};

use anyhow::{Result, anyhow};
use config::Config;
use serde_json::Value;

pub mod config;

pub fn run(config: Config) -> Result<()> {
    let file = File::open(&config.input)?;
    let reader = BufReader::new(file);
    let records: Value = serde_json::from_reader(reader)?;

    // Must be and array. Otherwise, return an error.
    if let Value::Array(mut records) = records {
        // Filter logic:
        if let Some(filters) = &config.filter {
            filters.iter().for_each(|filter| {
                records = records
                    .iter()
                    .cloned()
                    .filter(|record| {
                        if let Some(record) = record.get(filter.field.clone()) {
                            *record == filter.value
                        } else {
                            false
                        }
                    })
                    .collect();
            });
        }

        if config.count {
            println!("{}", &records.iter().count())
        } else {
            println!("{:#?}", &records);
        }
    } else {
        return Err(anyhow!("must be an array of records"));
    }

    Ok(())
}
