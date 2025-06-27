// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{SystemTime, UNIX_EPOCH};
use std::process;

use log::{info, error};
use simplelog::{Config, WriteLogger};

slint::include_modules!();

mod config;

fn main() {
    // Set the global launcher root directory constant
    config::initialize_launcher_root_directory();

    // Initialize log file every time the application starts
    let log_file_name = format!("{}.log",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    let log_file_path = config::get_launcher_root_directory().join("logs").join(log_file_name);
    let log_file = std::fs::File::create(&log_file_path).unwrap();

    WriteLogger::init(
        simplelog::LevelFilter::Info,
        Config::default(),
        log_file,
    ).unwrap();

    // Run main window
    info!("Starting LK Launcher");
    let app = MainWindow::new().unwrap_or_else( |e| {
        error!("Failed to initialize main window: {e}");
        process::exit(1);
    });

    if let Err(e) = app.run() {
        error!("Failed to run main window: {e}");
        process::exit(1);
    }

    info!("LK Launcher exited successfully");
}