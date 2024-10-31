use crate::asset::*;
use clap::{ArgGroup, Parser};

mod asset;
mod manifest;

#[derive(Parser, Debug)]
#[command(
    name = "xpak",
    version = "0.0.1-dev",
    about = "Asset packing tool for Xen Engine"
)]
#[command(group(
    ArgGroup::new("build_options")
        .required(true)
        .args(["build", "rebuild", "clean"]),
))]
struct Cli {
    /// The manifest file to read
    manifest: String,

    /// Build option
    #[arg(short, long)]
    build: bool,

    /// Rebuild option
    #[arg(short = 'r', long)]
    rebuild: bool,

    /// Clean option
    #[arg(short, long)]
    clean: bool,
}

fn main() {
    let asset = Asset::new("sprite/idle", "Texture", "sprite/idle.png");
    println!(
        "Name: {}, Type: {}, Source: {}",
        asset.name, asset.asset_type.to_string(), asset.source
    );

    // let cli = Cli::parse();

    // if cli.build {
    //     println!("Building...");
    //     // Handle build logic
    // } else if cli.rebuild {
    //     println!("Rebuilding...");
    //     // Handle rebuild logic
    // } else if cli.clean {
    //     println!("Cleaning...");
    //     // Handle clean logic
    // }
}
