// src/main.rs
mod config;
mod file_processor;
mod tag_processor;

use anyhow::Result;
use config::Config;
use file_processor::FileProcessor;
use log::{error, info};
use simple_logger::SimpleLogger;
use std::env;

fn main() -> Result<()> {
    SimpleLogger::new().init()?;
    info!("Starting tag cleaner");

    let config = load_config()?;
    let processor = FileProcessor::new(config);
    processor.process_paths();

    info!("Processing completed");
    Ok(())
}

fn load_config() -> Result<Config> {
    let args: Vec<String> = env::args().collect();
    let config_path = if args.len() > 1 && args[1] == "-c" {
        args[2].as_str()
    } else {
        "config.yaml"
    };

    Config::from_file(config_path).or_else(|_| {
        info!("Using default configuration");
        Ok(Config::default())
    })
}
