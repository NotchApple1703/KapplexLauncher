// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log;

mod core;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This will set the global launcher root directory constant.
    core::path::initialize()?; 

    // This will create a log file in the logs directory.
    // %Y-%m-%d_%H-%M-%S.log
    core::logging::setup()?;

    match MainWindow::new() {
        Ok(main_window) => {
            log::info!("Starting LK Launcher");
            main_window.run()?;
            log::info!("LK Launcher exited successfully");
        },
        Err(e) => {
            log::error!("Failed to initialize main window: {e}");
        }
    }

    Ok(())
}