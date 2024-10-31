use crate::asset::*;
use clap::{ArgGroup, Parser};
use crate::manifest::Manifest;

mod asset;
mod manifest;
mod processors;
mod build_cache;

#[derive(Parser, Debug)]
#[command(
    name = "xpak",
    version = "0.0.1-dev",
    about = "Asset packing tool for Xen Engine"
)]
#[command(group(
    ArgGroup::new("actions")
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
    let cli = Cli::parse();
    let mut manifest = Manifest::new(cli.manifest.as_str());

    if cli.build {
        println!("Building...");
        manifest.build();
    } else if cli.rebuild {
        println!("Rebuilding...");
        manifest.rebuild();
    } else if cli.clean {
        println!("Cleaning...");
        manifest.clean();
    }
}
