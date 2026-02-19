use std::process::exit;

use clap::Parser;
use logcruncher::config::Config;

fn main() {
    let config = Config::parse();

    if let Err(error) = logcruncher::run(config) {
        eprintln!("{}", error);
        exit(-1);
    }
}
