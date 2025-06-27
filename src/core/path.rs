use std::path::PathBuf;
use std::sync::OnceLock;
use std::fs;

static LAUNCHER_PATHS: OnceLock<LauncherPaths> = OnceLock::new();

#[derive(Debug)]
pub struct LauncherPaths {
    pub root_directory: PathBuf,
    pub logs_directory: PathBuf,
}

impl LauncherPaths {
    fn new() -> Result<Self, std::io::Error> {
        let launcher_executable_path = std::env::current_exe()?;

        let launcher_root_directory = launcher_executable_path.parent()
            .expect("Launcher executable must be in a directory")
            .to_path_buf();

        Ok(LauncherPaths {
            root_directory: launcher_root_directory.clone(),
            logs_directory: launcher_root_directory.join("logs"),
        })
    }
}

pub fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    LAUNCHER_PATHS.set(LauncherPaths::new()?)
        .expect("Launcher paths already initialized");

    for path in [
        &get().root_directory, 
        &get().logs_directory
    ] {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
    }

    Ok(())
}

pub fn get() -> &'static LauncherPaths {
    LAUNCHER_PATHS.get()
        .expect("Launcher paths not initialized")

}