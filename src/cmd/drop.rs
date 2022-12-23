use crate::cmd::RepoPath;
use crate::config::Config;
use cmd_lib::*;
use log::debug;
use std::error::Error;

/// Drop one or more repositories from the workspace
pub fn run_drop(config: &Config, path: Option<String>) -> Result<(), Box<dyn Error>> {
    debug!("Drop requested for: {:?}", &path);

    let repos = match path {
        Some(path) => RepoPath::parse(&config, path)?,
        None => Vec::new(),
    };

    for repo in repos {
        let p = repo.path();
        let out = run_fun!(git -C $p status --porcelain)?;
        println!("{}", out);
    }
    Ok(())
}
