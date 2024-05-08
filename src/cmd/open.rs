use crate::cmd::RepoPattern;
use crate::Config;
use anyhow::{bail, Result};
use cmd_lib::*;
use tracing::debug;

/// Open one or more repositories in the workspace
pub fn run_open(config: &Config, path: Option<String>) -> Result<()> {
    let path = match path {
        Some(path) => path,
        None => bail!("No pattern given"),
    };

    let pattern = match str::parse::<RepoPattern>(&path) {
        Ok(pattern) => pattern,
        Err(_) => bail!("Failed to parse pattern"),
    };

    debug!(pattern = ?pattern, "Opening repos");

    // Check the cache
    if let Some(cache) = &config.cache {
        // TODO cache.search
    }

    let repos = match path {
        Some(p) => config.search_local(&RepoPattern::parse(&p)?), // TODO remote
        None => Vec::new(),
    };

    for repo in repos {
        let out = run_fun!(git -C $repo status --porcelain)?;
        println!("{}", out);
    }

    Ok(())
}
