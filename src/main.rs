use crate::config::Config;
use clap::{Parser, Subcommand, ValueEnum};
use log::debug;
use std::error::Error;

pub mod api;
pub mod cmd;
pub mod config;

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
    Clone { path: String },

    #[command()]
    Drop { path: Option<String> },
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    env_logger::init();

    // Read configuration file
    let config_path = match std::env::home_dir() {
        Some(home) => format!("{}/.wsm/config.toml", home.display()),
        None => String::from(""),
    };
    let config: Config = match std::fs::read_to_string(config_path) {
        Ok(content) => toml::from_str(&content)?,
        Err(_) => todo!(),
    };

    debug!("Read user configuration: {:?}", &config);

    let args = Args::parse();

    match &args.command {
        Commands::Clone { path } => Ok(()),
        Commands::Drop { path } => crate::cmd::drop::run_drop(&config, path.clone()),
    }
}
