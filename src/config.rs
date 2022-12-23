use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents the user's config file
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub workspace: Vec<Workspace>,
}

/// Represents a workspace which is ultimately the thing we're managing
#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    /// A user-friendly name for the workspace like "personal" or "work"
    pub name: Option<String>,

    /// The workspace directory's filesystem path
    pub path: String,
    pub remote: Option<Vec<Remote>>,
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
pub struct Remote {
    /// The remote's name for use in repo paths
    pub name: String,
}
