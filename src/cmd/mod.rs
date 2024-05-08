use std::error::Error;
use std::str::FromStr;

use anyhow::bail;
use anyhow::Result;
use regex::Regex;

pub mod drop;
pub mod open;

/// Represents a pattern that matches one or more repositories. It has the
/// format: [workspace]:[remote]/[path].
#[derive(Debug, Eq, PartialEq)]
pub struct RepoPattern {
    /// The workspace name
    pub workspace_name: Option<String>,

    /// The repo path
    pub path: String,
}

impl FromStr for RepoPattern {
    type Err = Box<dyn Error>;

    fn from_str(path: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        match Regex::new(r"^([^/]+:)?(.*)$")?.captures(path) {
            Some(captures) => Ok(Self {
                workspace_name: captures
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .map(|s| s[..s.len() - 1].to_string()),
                path: captures.get(2).unwrap().as_str().to_string(),
            }),
            None => Err("Invalid repository path pattern".into()),
        }
    }
}

impl RepoPattern {
    /// Try to parse the remote from the repo path. A `RepoPattern` doesn't
    /// have enough information to know whether this is actually the remote, so
    /// leave that decision up to the caller.
    pub fn maybe_remote(&self) -> Option<(String, String)> {
        let parts: Vec<&str> = self.path.splitn(2, "/").collect();
        if parts.len() == 2 && parts[0] != "" {
            Some((parts[0].to_string(), parts[1].to_string()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_repo_pattern {
    use super::RepoPattern;
    use std::error::Error;

    #[test]
    fn test_parse() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            str::parse::<RepoPattern>("workspace12:remote1/abc/123")?,
            RepoPattern {
                workspace_name: Some("workspace12".to_string()),
                path: "remote1/abc/123".to_string()
            }
        );
        assert_eq!(
            str::parse::<RepoPattern>("123")?,
            RepoPattern {
                workspace_name: None,
                path: "123".to_string()
            }
        );

        Ok(())
    }
}
