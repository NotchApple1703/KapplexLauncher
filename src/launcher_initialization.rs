use std::{env, fs};

pub fn initialize_launcher_root_directory() {
    let launcher_executable_path = env::current_exe().unwrap();
    let launcher_root_directory = launcher_executable_path.parent().unwrap();

    [
        "assets",
        "cache",
        "icons",
        "instances",
        "java",
        "libraries",
        "logs",
        "themes",
        "translations"
    ]
    .iter()
    .for_each( |directory| {
        let _ = fs::create_dir(launcher_root_directory.join(directory));
    });
}