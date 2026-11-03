mod config;
mod filter;
mod keys;
mod model;
mod palette;
mod png;
mod theme;
mod tui;
mod ui;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Write the cheat sheet as a PNG image
    Png {
        /// Output file
        #[arg(short, long, default_value = "cheat_sheet.png")]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let apps = match config::load() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("shortcheat: {e}");
            std::process::exit(1);
        }
    };

    match cli.command {
        None => tui::start(apps),
        Some(Command::Png { output }) => {
            if let Err(e) = png::generate(&apps, &output) {
                eprintln!("shortcheat: {e:#}");
                std::process::exit(1);
            }
            println!("Saved cheat sheet to {}", output.display());
            Ok(())
        }
    }
}
