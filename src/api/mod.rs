use anyhow::Result;
use std::fmt::Display;

//pub mod github;
//pub mod gitlab;

pub trait Provider: Display {
    /// List all repository paths available to the provider.
    fn list_repo_paths(&self) -> Result<Vec<String>>;
}
