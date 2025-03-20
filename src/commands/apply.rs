use anyhow::Result;

use super::super::config::{Options, StackConfig};
use super::super::git::{apply_patches as git_apply_patches, enter_detached};
use super::super::series::*;

fn process_stack(base_path: &std::path::Path, stack_config: &StackConfig) -> Result<()> {
    let mut src_dir = base_path.to_path_buf();
    let output_dir = &stack_config.output_directory;

    src_dir.push(stack_config.src_path.clone());

    // Read in series file
    let mut series = Series::new(output_dir);
    series.populate()?;

    git_apply_patches(output_dir, series.patch_files())
}

pub fn apply_patches(base_path: &std::path::Path, options: &Options) -> Result<()> {
    // Make sure we go into a detached head state to not confuse the
    // base branch with a branch that has patches applied
    enter_detached()?;

    for stack in &options.stacks {
        process_stack(base_path, stack)?;
    }

    Ok(())
}
