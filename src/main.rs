use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct Config {}

#[derive(Debug, Parser)]
#[command(name = "wsm")]
#[command(about = "Cilki's WorkSpace Manager", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Clone {
        path: String,
    },

    #[command()]
    Drop { path: Option<String> },
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read configuration file
    let config_path = match std::env::home_dir() {
        Some(home) => format!("{}/.wsm/config.toml", home.display()),
        None => String::from(""),
    };
    let config = match std::fs::read_to_string(config_path) {
        Ok(content) => toml::from_str(&content)?,
        Err(_) => todo!(),
    };

    let args = Args::parse();

    match &args.command {
        Commands::Clone { path } => Ok(()),
        Commands::Drop { path } => cmd_drop(&config, path.clone()),
    }
}

fn cmd_drop(config: &Config, path: Option<String>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
