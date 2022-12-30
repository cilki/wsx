use crate::cmd::RepoPattern;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::path::PathBuf;

/// Represents the user's config file
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub workspace: Vec<WorkspaceConfig>,
}

impl Config {
    /// Resolve a repository pattern against locally checked-out repositories.
    pub fn resolve(&self, pattern: &RepoPattern) -> Vec<PathBuf> {
        let workspace: &WorkspaceConfig = match &pattern.workspace {
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

        match pattern.maybe_provider() {
            Some((provider, path)) => {}
            None => {}
        }

        todo!()
    }
}

/// Represents a workspace which is ultimately the thing we're managing
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// A user-friendly name for the workspace like "personal" or "work"
    pub name: Option<String>,

    /// The workspace directory's filesystem path
    pub path: String,

    /// The cache directory for the workspace
    #[serde(default = "default_cache")]
    pub cache: Option<String>,

    /// A list of providers for the workspace
    pub providers: Option<Vec<ProviderConfig>>,
}

fn default_cache() -> Option<String> {
    Some("~/.wsm/cache".to_string())
}

impl WorkspaceConfig {
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
pub struct ProviderConfig {
    /// The provider's name for use in repo paths
    pub name: String,
}
