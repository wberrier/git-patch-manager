use anyhow::Result;
use std::process::Command;

use super::super::config::{find_root_dir, get_options, StackConfig};

fn process_stack(base_path: &std::path::PathBuf, stack_config: &StackConfig) -> Result<()> {
    let mut src_dir = base_path.clone();
    let output_dir = stack_config.output_directory.clone();

    src_dir.push(stack_config.src_path.clone());

    // Generate all the patches from this directory
    //let command = format!("git format-patch -o {} {}", output_dir.into(), src_dir.into());

    // TODO: execute the command

    // Generate the series file
    // NOTE: is the series file necessary?  Likely, to control which patches get applied

    Ok(())
}

pub fn generate_patches() -> Result<()> {
    let options = get_options()?;

    let base_path = find_root_dir()?;

    for stack in &options.stacks {
        process_stack(&base_path, &stack)?;
    }

    Ok(())
}
