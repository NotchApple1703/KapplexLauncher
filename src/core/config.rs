use std::path::PathBuf;
use std::sync::OnceLock;

static LAUNCHER_ROOT_DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

pub fn initialize_launcher_root_directory() {
    let launcher_executable_path = std::env::current_exe().unwrap();
    let launcher_root_directory = launcher_executable_path
        .parent()
        .unwrap()
        .to_path_buf();

    LAUNCHER_ROOT_DIRECTORY.set(launcher_root_directory).unwrap();
}

pub fn get_launcher_root_directory() -> &'static PathBuf {
    return LAUNCHER_ROOT_DIRECTORY.get().unwrap();
}