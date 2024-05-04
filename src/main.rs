use anyhow::Result;
use tracing::debug;
use wsm::Config;

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
    let config_path = match std::env::home_dir() {
        Some(home) => format!("{}/.wsm/config.toml", home.display()),
        None => todo!(),
    };
    debug!(config_path = %config_path, "Loading configuration file");

    // Read configuration file
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
            "drop" => wsm::cmd::drop::run_drop(&config, args.opt_free_from_str()?),
            "help" => print_help(),
            _ => wsm::cmd::open::run_open(&config, Some(command)),
        },
        None => todo!(), // TODO open UI
    }
}

/// Output help text.
fn print_help() -> Result<()> {
    println!(
        "wsm {} ({})",
        build_info::PKG_VERSION,
        build_info::BUILT_TIME_UTC
    );
    println!("");
    println!("wsm <repo pattern>         - Clone one or more repositories");
    println!("wsm drop [repo pattern]    - Drop one or more repositories");
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
