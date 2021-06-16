use anyhow::{bail, Result};

use serde::Deserialize;
use std::env;
// Define a config file

const CONFIG_FILE_NAME: &str = "git-pm.yaml";

#[derive(Deserialize)]
pub struct StackConfig {
    pub src_path: std::path::PathBuf,
    pub output_directory: std::path::PathBuf,
}

#[derive(Deserialize)]
pub struct Options {
    pub base_branch: String,
    pub stacks: Vec<StackConfig>,
}

pub fn find_root_dir() -> Result<std::path::PathBuf> {
    let mut current_dir = env::current_dir()?;

    while current_dir != std::path::Path::new("/") {
        let mut test = current_dir.clone();
        test.push(CONFIG_FILE_NAME);
        if test.exists() {
            test.pop();
            return Ok(test);
        }

        current_dir.pop();
    }

    bail!("unable to find project root")
}

fn find_config_file() -> Result<std::path::PathBuf> {
    let mut config_file_path = find_root_dir()?;
    config_file_path.push(CONFIG_FILE_NAME);
    Ok(config_file_path)
}

pub fn get_options() -> Result<Options> {
    let config_file = find_config_file()?;

    let yaml_config = std::fs::read_to_string(config_file)?;

    let options = serde_yaml::from_str(&yaml_config)?;

    Ok(options)
}
