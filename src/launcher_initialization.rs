use std::fs;

pub fn initialize_launcher_directory() {
    let directories = [
        "assets",
        "cache",
        "icons",
        "instances",
        "java",
        "libraries",
        "logs",
        "themes",
        "translations"
    ];

    directories.iter().for_each(
        |directory| fs::create_dir(directory).unwrap()
    );
}