// cli.rs

use colored::Colorize;
use crate::dir::Dir;
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "emojirs_ls",
    version,
    about = "ls with emojis",
    about = "Jibril"
)]
struct Args {
    #[arg(default_value = ".")]
    directory: String,
    #[arg(short, long, help = "Show hidden files")]
    all: bool,
}

pub fn cli() {
    let args = Args::parse();
    let dir = Dir { dotfiles: args.all };
    if let Err(e) = dir.list(args.directory) {
        eprintln!("{}{}", "Error: ".bright_red(), e.to_string().bright_red());
    }
}