use std::{env, fs, path::PathBuf};

use crate::agents::utils::merge_agents;
use crate::config::{
    constants::{DEFAULT_AGENTS, DEFAULT_PERMISSIONS, FOLDER_NAME},
    types::Config,
};

pub fn get_config_file() -> PathBuf {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .expect("Could not find home directory");

    let config_dir = PathBuf::from(home).join(".tiny-rick");

    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    let config_file = config_dir.join(format!("{}.json", FOLDER_NAME.to_string()));

    if !config_file.exists() {
        fs::write(&config_file, "{}").expect("Failed to write default config");
    }

    config_file
}

pub fn load_or_create_config() -> Config {
    let config_file_path = get_config_file();

    let data = fs::read_to_string(config_file_path).unwrap_or_else(|_| "{}".to_string());

    let mut config: Config = serde_json::from_str(&data).unwrap_or_else(|_| Config {
        providers: vec![],
        permissions: DEFAULT_PERMISSIONS.to_vec(),
        agents: DEFAULT_AGENTS.to_vec(),
        ui: vec![],
    });

    config.agents = merge_agents(config.agents);

    config
}
