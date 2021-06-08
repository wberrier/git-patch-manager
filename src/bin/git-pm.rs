use structopt::StructOpt;

use anyhow::Result;

use git_patch_manager::commands::generate::generate_patches;

#[derive(Debug, StructOpt)]
#[structopt(about, author)]
enum PatchManagerCommands {
    #[structopt(about = "generate patches")]
    Generate {},
}

fn main() -> Result<()> {
    let subcommand_options = PatchManagerCommands::from_args();

    // Handle the info sub command
    match subcommand_options {
        PatchManagerCommands::Generate {} => generate_patches(),
    }
}
