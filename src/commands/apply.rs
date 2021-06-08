use anyhow::Result;

use super::super::config::{find_root_dir, get_options, StackConfig};
use super::super::git::{apply_patches as git_apply_patches, enter_detached};
use super::super::series::*;

fn process_stack(base_path: &std::path::Path, stack_config: &StackConfig) -> Result<()> {
    let mut src_dir = base_path.to_path_buf();
    let output_dir = &stack_config.output_directory;

    src_dir.push(stack_config.src_path.clone());

    // Read in series file
    let mut series = Series::new(output_dir);
    series.from_file()?;

    git_apply_patches(series.patch_files())
}

// Can I apply more than one patch at a time?  (ie: would that be faster?)

pub fn apply_patches() -> Result<()> {
    let options = get_options()?;

    let base_path = find_root_dir()?;

    // Make sure we go into a detached head state to not confuse the
    // base branch with a branch that has patches applied
    enter_detached()?;

    for stack in &options.stacks {
        process_stack(&base_path, &stack)?;
    }

    Ok(())
}
