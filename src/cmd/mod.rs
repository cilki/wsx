use crate::config::Config;
use std::error::Error;

pub mod drop;

pub struct RepoPath {
    /// The workspace name
    pub workspace: Option<String>,

    /// The remote name
    pub remote: Option<String>,

    /// The repo parent path
    pub path: Option<String>,

    /// The repo name
    pub name: String,
}

impl RepoPath {
    pub fn parse(config: &Config, path: String) -> Result<Vec<Self>, Box<dyn Error>> {
        let url = url::Url::parse(&path)?;

        // Parse the workspace if one is given
        let workspace = if url.scheme() == "" {
            // If there's only one workspace configured, that must be the one
            if config.workspace.len() == 1 {
                Some(config.workspace[0].name())
            } else {
                None
            }
        } else {
            Some(url.scheme().to_string())
        };

        // Parse the path

        Ok(vec![])
    }

    pub fn path(&self) -> String {
        match &self.path {
            Some(path) => format!("{}/{}", path, self.name),
            None => self.name.clone(),
        }
    }
}
