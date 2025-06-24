use std::collections::HashMap;
use std::path::PathBuf;

fn convert_libraries_path(libraries_path: HashMap<&str, &str>) -> HashMap<String, PathBuf> {
    libraries_path
        .iter()
        .map( |(k, v)| (k.to_string(), PathBuf::from(v)) )
        .collect()
}

fn main() {
    let libraries_path = convert_libraries_path(HashMap::from([
        ("style", "ui/style"),
        ("components", "ui/components")
    ]));

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(libraries_path);

    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
