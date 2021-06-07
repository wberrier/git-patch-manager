use anyhow::{anyhow, Result};

use serde::Deserialize;
use std::env;
// Define a config file

const CONFIG_FILE_NAME: &str = "git-pm.yaml";

#[derive(Deserialize)]
struct StackConfig {
    src_path: std::path::PathBuf,
    output_directory: std::path::PathBuf,
}

#[derive(Deserialize)]
pub struct Options {
    base_branch: String,
    stacks: Vec<StackConfig>,
}

fn find_config_file() -> Result<std::path::PathBuf> {
    let mut current_dir = env::current_dir()?;

    while current_dir != std::path::Path::new("/") {
        let mut test = current_dir.clone();
        test.push(CONFIG_FILE_NAME);
        if test.exists() {
            return Ok(test);
        }

        current_dir.pop();
    }

    Err(anyhow!("unable to find project root"))
}

pub fn get_options() -> Result<Options> {
    let config_file = find_config_file()?;

    let yaml_config = std::fs::read_to_string(config_file)?;

    let options = serde_yaml::from_str(&yaml_config)?;

    Ok(options)
}
