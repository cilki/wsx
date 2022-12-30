use crate::config::Config;
use log::debug;
use std::error::Error;

pub mod api;
pub mod cache;
pub mod cmd;
pub mod config;

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

    match std::env::var("_ARGCOMPLETE_") {
        Ok(shell_type) => {
            return match shell_type.as_str() {
                "bash" => complete_bash(),
                "fish" => complete_fish(),
                _ => todo!(),
            };
        }
        Err(_) => (),
    }

    let mut args = pico_args::Arguments::from_env();

    match args.subcommand()? {
        Some(command) => match command.as_str() {
            "checkout" => todo!(),
            "drop" => crate::cmd::drop::run_drop(&config, args.opt_free_from_str()?),
            _ => todo!(),
        },
        None => todo!(),
    }
}

/// Output dynamic completions for bash
fn complete_bash() -> Result<(), Box<dyn Error>> {
    todo!()
}

/// Output dynamic completions for fish
fn complete_fish() -> Result<(), Box<dyn Error>> {
    todo!()
}
