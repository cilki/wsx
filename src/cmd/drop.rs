use crate::cmd::RepoPattern;
use crate::Config;
use anyhow::Result;
use cmd_lib::*;
use log::debug;

/// Drop one or more repositories from the workspace
pub fn run_drop(config: &Config, path: Option<String>) -> Result<()> {
    debug!("Drop requested for: {:?}", &path);

    let repos = match path {
        Some(p) => config.resolve_local(&RepoPattern::parse(&p)?),
        None => Vec::new(),
    };

    for repo in repos {
        let out = run_fun!(git -C $repo status --porcelain)?;
        if out == "" {
            if let Some(cache) = &config.cache {
                // Cache the repository
                cache.cache(repo.to_string_lossy().to_string())?;
            }

            // Remove the directory
            debug!("Removing directory: {:?}", &repo);
            std::fs::remove_dir_all(repo)?;
        } else {
            debug!("Refusing to drop repository with uncommitted changes");
        }
    }
    Ok(())
}
