// Put all git commands in here

// If necessary, eventually we could use git2 crate for a more programmatic approach

use anyhow::{bail, Result};

use crate::subprocess::*;

// Return a list of generated patches
pub fn format_patches(
    base_branch: &str,
    src_dir: &std::path::Path,
    output_dir: &std::path::Path,
) -> Result<Vec<std::path::PathBuf>> {
    // Generate all the patches from this directory
    let command = format!(
        "git format-patch -o {} {} -- {}",
        output_dir.to_string_lossy(),
        base_branch,
        src_dir.to_string_lossy()
    );

    let (status, output) = getstatusoutput(command.as_str())?;

    print!("{}", output);

    if !status.success() {
        bail!("Error running {}", command);
    }

    let mut patches = Vec::<std::path::PathBuf>::new();

    for line in output.split('\n') {
        if !line.is_empty() {
            patches.push(std::path::PathBuf::from(line));
        }
    }

    Ok(patches)
}

pub fn apply_patches(
    patch_directory: &std::path::Path,
    patch_files: &[std::path::PathBuf],
) -> Result<()> {
    // Use 3way patch apply by default
    let mut command = "git am --3way --whitespace=nowarn --ignore-whitespace".to_string();

    // Is it any faster to apply more than one patch at a time?
    for p in patch_files {
        command.push_str(format!(" {}/{}", patch_directory.display(), p.display()).as_str());
    }

    let status = getstatus(command.as_str())?;

    if !status.success() {
        bail!("Error running {}", command);
    }

    Ok(())
}

pub fn enter_detached() -> Result<()> {
    run_or_fail("git checkout --detach")?;
    Ok(())
}
