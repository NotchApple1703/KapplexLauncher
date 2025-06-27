use std::fs::File;

use chrono::Local;
use log::LevelFilter;
use simplelog::{Config, WriteLogger};

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let logs_directory = &crate::core::path::get().logs_directory;
    
    let log_file_name = format!("{}.log",
        Local::now().format("%Y-%m-%d_%H-%M-%S")
    );

    let log_file_path = logs_directory.join(log_file_name);

    let log_file = File::create(&log_file_path)
        .map_err(|e| format!("Failed to create log file: {e}"))?;

    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        log_file,
    ).map_err(|e| format!("Failed to initialize logger: {e}"))?;

    Ok(())
}