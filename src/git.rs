// Put all git commands in here

// If necessary, eventually we could use git2 crate for a more programmatic approach

use anyhow::{anyhow, Result};

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
        return Err(anyhow!("Error running {}", command));
    }

    let mut patches = Vec::<std::path::PathBuf>::new();

    for line in output.split('\n') {
        if !line.is_empty() {
            patches.push(std::path::PathBuf::from(line));
        }
    }

    Ok(patches)
}

pub fn apply_patches(patch_files: &Vec<std::path::PathBuf>) -> Result<()> {
    let mut command = "git am".to_string();

    for p in patch_files {
        command.push_str(format!(" {}", p.display()).as_str());
    }

    let status = getstatus(command.as_str())?;

    if !status.success() {
        return Err(anyhow!("Error running {}", command));
    }

    Ok(())
}

pub fn enter_detached() -> Result<()> {
    run_or_fail("git checkout --detach")?;
    Ok(())
}
