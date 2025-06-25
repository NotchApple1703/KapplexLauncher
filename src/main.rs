// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod launcher_initialization;

slint::include_modules!();

fn main() {
    launcher_initialization::initialize_launcher_root_directory();

    let app = MainWindow::new().unwrap();
    app.run().unwrap();
}