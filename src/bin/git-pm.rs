use clap::Parser;

use anyhow::Result;

use git_patch_manager::commands::apply::apply_patches;
use git_patch_manager::commands::generate::generate_patches;

use git_patch_manager::config::{find_root_dir, get_options};

#[derive(Debug, Parser)]
#[clap(about, author)]
enum PatchManagerCommands {
    #[clap(about = "generate patches")]
    Generate {},
    #[clap(about = "apply patches")]
    Apply {},
}

fn main() -> Result<()> {
    let subcommand_options = PatchManagerCommands::parse();

    // Change to the repo root
    let base_path = find_root_dir()?;
    std::env::set_current_dir(&base_path)?;

    let options = get_options()?;

    // Handle the info sub command
    match subcommand_options {
        PatchManagerCommands::Generate {} => generate_patches(&base_path, &options),
        PatchManagerCommands::Apply {} => apply_patches(&base_path, &options),
    }
}
