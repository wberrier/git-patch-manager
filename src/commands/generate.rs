use anyhow::{bail, Result};

use super::super::config::{Options, StackConfig};
use super::super::git::format_patches;
use super::super::series::*;

fn process_stack(
    base_path: &std::path::Path,
    stack_config: &StackConfig,
    base_branch: &str,
) -> Result<()> {
    let mut src_dir = base_path.to_path_buf();
    let output_dir = &stack_config.output_directory;

    src_dir.push(stack_config.src_path.clone());

    let patches = format_patches(base_branch, &src_dir, output_dir)?;

    // Generate the series file (if there are any)
    if !patches.is_empty() {
        let mut series = Series::new(output_dir);

        // Patches are relative to the series file, save just the filename
        let new_patches: Vec<std::path::PathBuf> = patches
            .iter()
            .map(|patch| {
                std::path::PathBuf::from(
                    patch
                        .file_name()
                        .expect("Error getting filename from patch path"),
                )
            })
            .collect();

        series.set_patch_files(&new_patches)?;
        series.persist()?;
    } else {
        // Make sure that some patches were generated
        bail!("no patches generated from: {:?}", stack_config.src_path)
    }

    Ok(())
}

pub fn generate_patches(base_path: &std::path::Path, options: &Options) -> Result<()> {
    for stack in &options.stacks {
        process_stack(base_path, stack, &options.base_branch)?;
    }

    Ok(())
}
