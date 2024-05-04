use anyhow::Result;
use tracing::debug;
use wsx::Config;

/// Build info provided by built crate.
pub mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Locate configuration file
    let config_path = match home::home_dir() {
        Some(home) => format!("{}/.config/wsx.toml", home.display()),
        None => todo!(),
    };

    // Load configuration file
    let config: Config = match std::fs::metadata(&config_path) {
        Ok(_) => {
            debug!(config_path = %config_path, "Loading configuration file");
            toml::from_str(&std::fs::read_to_string(config_path)?)?
        }
        Err(_) => {
            debug!("Using default config");
            Config::default()
        }
    };

    debug!(config = ?config, "Using configuration");

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
    if args.contains("--help") {
        return print_help();
    }

    match args.subcommand()? {
        Some(command) => match command.as_str() {
            "drop" => wsx::cmd::drop::run_drop(&config, args.opt_free_from_str()?),
            "help" => print_help(),
            _ => wsx::cmd::open::run_open(&config, Some(command)),
        },
        None => todo!(), // TODO open UI
    }
}

/// Output help text.
fn print_help() -> Result<()> {
    println!(
        "wsx {} ({})",
        build_info::PKG_VERSION,
        build_info::BUILT_TIME_UTC
    );
    println!("");
    println!("wsx <repo pattern>         - Clone one or more repositories");
    println!("wsx drop [repo pattern]    - Drop one or more repositories");
    Ok(())
}

/// Output dynamic completions for bash
fn complete_bash() -> Result<()> {
    todo!()
}

/// Output dynamic completions for fish
fn complete_fish() -> Result<()> {
    todo!()
}
