use anyhow::Result;

use super::super::config::{find_root_dir, get_options, StackConfig};
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

    let patches = format_patches(base_branch, &src_dir, &output_dir)?;

    // Generate the series file
    let series = Series::new(output_dir, patches);
    series.to_file()?;

    Ok(())
}

pub fn generate_patches() -> Result<()> {
    let options = get_options()?;

    let base_path = find_root_dir()?;

    for stack in &options.stacks {
        process_stack(&base_path, &stack, &options.base_branch)?;
    }

    Ok(())
}
