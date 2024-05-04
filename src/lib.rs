use crate::cmd::RepoPattern;
use anyhow::Result;
use cmd_lib::run_fun;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::path::Path;
use std::path::PathBuf;
use tracing::debug;

pub mod api;
pub mod cmd;

/// Represents the user's config file
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub workspace: Vec<Workspace>,

    /// The cache directory for all workspaces
    #[serde(flatten)]
    pub cache: Option<Cache>,
}

impl Default for Config {
    // Place the cache according to platform

    fn default() -> Self {
        Self {
            workspace: vec![],
            cache: None,
        }
    }
}

impl Config {
    /// Resolve a repository pattern against local repositories.
    pub fn resolve_local(&self, pattern: &RepoPattern) -> Vec<PathBuf> {
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

        let (provider, path) = match pattern.maybe_provider() {
            Some((provider, path)) => {
                debug!("{} {}", provider, path);
                (
                    workspace
                        .provider
                        .iter()
                        .find(|&p| p.name == provider)
                        .unwrap(),
                    path,
                )
            }
            None => (workspace.provider.first().unwrap(), pattern.path.clone()),
        };

        find_git_dir(&format!("{}/{}/{}", workspace.path, provider.name, path)).unwrap()
    }
}

/// Recursively find git repositories.
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

/// Represents a workspace which is just a collection of repositories.
#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// A user-friendly name for the workspace like "personal" or "work"
    pub name: Option<String>,

    /// The workspace directory's filesystem path
    pub path: String,

    /// A list of providers for the workspace
    pub provider: Vec<Provider>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    /// The provider's name for use in repo paths
    pub name: String,
}

/// Represents a workspace cache which is just a collection of bare repositories.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub cache: String,
}

impl Cache {
    /// Move the given repository into the cache.
    pub fn cache(&self, repo_path: String) -> Result<()> {
        // Make sure the cache directory exists first
        std::fs::create_dir_all(&self.cache)?;

        let source = format!("{}/.git", repo_path);
        let dest = self.compute_cache_key(&repo_path);
        run_fun!(git -C $source config core.bare true)?;

        debug!("Caching '{}' -> '{}'", source, dest);

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
            self.cache,
            Sha512::new().chain_update(path).finalize()
        )
    }
}
