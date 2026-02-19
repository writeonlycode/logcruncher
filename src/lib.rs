use anyhow::Result;
use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<()> {
    println!("{:?}", config);
    Ok(())
}
