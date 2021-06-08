use structopt::StructOpt;

use anyhow::Result;

use git_patch_manager::commands::apply::apply_patches;
use git_patch_manager::commands::generate::generate_patches;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
enum PatchManagerCommands {
    #[structopt(about = "generate patches")]
    Generate {},
    #[structopt(about = "apply patches")]
    Apply {},
}

fn main() -> Result<()> {
    let subcommand_options = PatchManagerCommands::from_args();

    // Handle the info sub command
    match subcommand_options {
        PatchManagerCommands::Generate {} => generate_patches(),
        PatchManagerCommands::Apply {} => apply_patches(),
    }
}
