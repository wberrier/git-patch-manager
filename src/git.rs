// Put all git commands in here

// If necessary, eventually we could use git2 crate for a more programmatic approach

use anyhow::Result;

use shleazy::*;

// Return a list of generated patches
pub fn format_patches(
    base_branch: &str,
    src_dir: &std::path::Path,
    output_dir: &std::path::Path,
) -> Result<Vec<std::path::PathBuf>> {
    // Generate all the patches from this directory
    let command = format!(
        "git format-patch --no-numbered --no-signature --zero-commit --no-stat -o {} {} -- {}",
        output_dir.to_string_lossy(),
        base_branch,
        src_dir.to_string_lossy()
    );

    let output = getoutput_shell_or_err(command)?;

    print!("{:?}", output);

    let mut patches = Vec::<std::path::PathBuf>::new();

    for line in String::from_utf8_lossy(&output).split('\n') {
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
    let mut command =
        "git am --committer-date-is-author-date --3way --whitespace=nowarn --ignore-whitespace"
            .to_string();

    // Is it any faster to apply more than one patch at a time?
    for p in patch_files {
        command.push_str(format!(" {}/{}", patch_directory.display(), p.display()).as_str());
    }

    run_shell_or_err(command)
}

pub fn enter_detached() -> Result<()> {
    run_shell_or_err("git checkout --detach")
}
