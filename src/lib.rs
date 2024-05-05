use crate::cmd::RepoPattern;
use anyhow::bail;
use anyhow::Result;
use cmd_lib::run_fun;
use remote::Remote;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::path::Path;
use std::path::PathBuf;
use tracing::debug;

pub mod cmd;
pub mod remote;

/// Represents the user's config file
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub workspace: Vec<Workspace>,

    /// The cache directory for all workspaces
    #[serde(flatten)]
    pub cache: Option<RepoCache>,
}

impl Default for Config {
    // Place the cache according to platform

    fn default() -> Self {
        let home = home::home_dir().expect("the home directory exists");

        Self {
            workspace: vec![Workspace {
                name: Some("default".into()),
                path: home.join("workspace").display().to_string(),
                remotes: vec![],
            }],
            cache: Some(RepoCache {
                path: home.join(".cache/wsx").display().to_string(),
            }),
        }
    }
}

impl Config {
    /// Load the application config from the filesystem or provide a default if
    /// none exists.
    pub fn load() -> Result<Self> {
        let config_path = match home::home_dir() {
            Some(home) => format!("{}/.config/wsx.toml", home.display()),
            None => bail!("Home directory not found"),
        };
        debug!(config_path = %config_path, "Searching for configuration file");

        let config: Config = match std::fs::metadata(&config_path) {
            Ok(_) => toml::from_str(&std::fs::read_to_string(config_path)?)?,
            Err(_) => Config::default(),
        };
        debug!(config = ?config, "Loaded configuration");

        // Make sure all necessary directories exist
        if let Some(cache) = config.cache.as_ref() {
            std::fs::create_dir_all(&cache.path)?;
        }
        for workspace in config.workspace.iter() {
            std::fs::create_dir_all(&workspace.path)?;
        }

        Ok(config)
    }

    /// Resolve a repository pattern against local repositories.
    pub fn resolve_local(&self, pattern: &RepoPattern) -> Vec<PathBuf> {
        // Either find the specified workspace or choose the first available
        let workspace: &Workspace = match &pattern.workspace {
            Some(workspace_name) => self
                .workspace
                .iter()
                .find(|&w| match &w.name {
                    Some(name) => name == workspace_name,
                    None => false,
                })
                .unwrap(),
            None => self.workspace.first().unwrap(),
        };

        let (remote, path) = match pattern.maybe_provider() {
            Some((provider, path)) => {
                debug!("{} {}", provider, path);
                (
                    workspace
                        .remotes
                        .iter()
                        .find(|&p| p.name == provider)
                        .unwrap(),
                    path,
                )
            }
            None => (workspace.remotes.first().unwrap(), pattern.path.clone()),
        };

        find_git_dir(&format!("{}/{}/{}", workspace.path, remote.name, path)).unwrap()
    }
}

/// Recursively find "top-level" git repositories.
fn find_git_dir(path: &str) -> Result<Vec<PathBuf>> {
    debug!("Searching for git repositories in: {}", path);
    let mut found: Vec<PathBuf> = Vec::new();

    match std::fs::metadata(format!("{}/.git", &path)) {
        Ok(_) => found.push(PathBuf::from(path)),
        Err(_) => {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if std::fs::metadata(&path)?.is_dir() {
                    found.append(&mut find_git_dir(&path.to_string_lossy())?);
                }
            }
        }
    }

    Ok(found)
}

/// A `Workspace` is filesystem directory containing git repositories checked out
/// from one or more remotes. Each repository's path matches the remote's path,
/// for example:
///     <workspace path>/github.com/cilki/wsx
#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// A user-friendly name for the workspace like "personal" or "work"
    pub name: Option<String>,

    /// The workspace directory's filesystem path
    pub path: String,

    /// A list of providers for the workspace
    pub remotes: Vec<Remote>,
}

impl Workspace {
    /// Get a user-friendly name for the workspace
    pub fn name(&self) -> String {
        match &self.name {
            Some(name) => String::from(name),
            None => Path::new(&self.path)
                .file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap(),
        }
    }
}

/// Caches repositories that are dropped from a `Workspace` in a separate directory.
/// Entries in this cache are bare repositories for space efficiency.
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoCache {
    pub path: String,
    // TODO cache parameters?
}

impl RepoCache {
    /// Move the given repository into the cache.
    pub fn cache(&self, repo_path: String) -> Result<()> {
        // Make sure the cache directory exists first
        std::fs::create_dir_all(&self.path)?;

        let source = format!("{}/.git", repo_path);
        let dest = self.compute_cache_key(&repo_path);
        run_fun!(git -C $source config core.bare true)?;

        debug!(source = %source, dest = %dest, "Caching repository");

        // Clear the cache entry if it exists
        std::fs::remove_dir_all(&dest).ok();

        run_fun!(mv $source $dest)?;
        Ok(())
    }

    pub fn uncache(&self, repo_path: String) -> Result<()> {
        let source = self.compute_cache_key(&repo_path);
        run_fun!(git clone $source $repo_path)?;
        Ok(())
    }

    pub fn exists(&self, repo_path: String) -> bool {
        match std::fs::metadata(self.compute_cache_key(&repo_path)) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn compute_cache_key(&self, path: &str) -> String {
        format!(
            "{}/{:x}",
            self.path,
            Sha512::new().chain_update(path).finalize()
        )
    }
}
