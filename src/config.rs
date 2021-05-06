use directories_next::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub aliases: Vec<Alias>
}

#[derive(Deserialize, Serialize)]
pub struct Alias {
    pub name: String,
    pub ori: String,
    pub args: String
}

pub fn read_config() {
    let dir = ProjectDirs::from("shell", "shell", "alfred")
        .expect("Unable to create project dir");

    let dir = dir.config_dir();
    if !Path::exists(dir) {
        println!("Config does not exist. Proceeding with default config.");
    }

    let file = Path::new("config.json");
    let dir = dir.join(file);
}

pub fn process_aliases(dir: PathBuf) -> Vec<Alias> {
    let file_content = fs::read_to_string(dir).expect("Unable to read file.");

    let json: Config = serde_json::from_str(&file_content).expect("Unable to parse JSON.");
    json.aliases
}
