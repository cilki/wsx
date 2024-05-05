use self::github::GithubRemote;
use anyhow::Result;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub mod github;
pub mod gitlab;

#[enum_dispatch]
pub trait ListRepos {
    /// List all repository paths available to the provider.
    fn list_repo_paths(&self) -> Result<Vec<String>>;
}

#[derive(Debug, Serialize, Deserialize)]
#[enum_dispatch(ListRepos)]
pub enum Remote {
    Github(GithubRemote),
    // Gitlab(GitlabRemote),
}
