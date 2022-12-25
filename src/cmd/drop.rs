use crate::cmd::RepoPattern;
use crate::config::Config;
use cmd_lib::*;
use log::debug;
use std::error::Error;

/// Drop one or more repositories from the workspace
pub fn run_drop(config: &Config, path: Option<String>) -> Result<(), Box<dyn Error>> {
    debug!("Drop requested for: {:?}", &path);

    let repos = match path {
        Some(p) => config.resolve(&RepoPattern::parse(&p)?),
        None => Vec::new(),
    };

    for repo in repos {
        let out = run_fun!(git -C $repo status --porcelain)?;
        println!("{}", out);
    }
    Ok(())
}
