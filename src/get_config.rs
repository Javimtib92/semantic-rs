use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
};

const CONFIG_NAME: &str = "release";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub branches: Vec<String>,
    pub ci: bool,
    pub debug: bool,
    pub dry_run: bool,
    pub repository_url: String,
    pub tag_format: String,
}

pub fn find_config(config_file_name: &str) -> Option<(PathBuf, Config)> {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let extensions = ["toml"];

    for dir in current_dir.ancestors() {
        for ext in extensions {
            let file_name = format!("{}.{}", config_file_name, ext);
            let file_path = dir.join(&file_name);

            if file_path.is_file() {
                let contents =
                    fs::read_to_string(&file_path).expect("Should have been able to read the file");

                if ext == "toml" {
                    let config: Config =
                        toml::from_str(contents.as_str()).expect("Couldn\'t deserialize config.");

                    return Some((file_path, config));
                }
            }
        }
    }

    None
}

pub fn get_config() -> Option<Config> {
    if let Some((_, config)) = find_config(CONFIG_NAME) {
        Some(config)
    } else {
        None
    }
}
