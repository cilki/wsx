use regex::Regex;
use simple_error::bail;
use std::error::Error;

pub mod drop;

/// Represents a pattern that matches one or more repositories. It has the
/// format: [workspace]:[remote]/[path]/[name].
#[derive(Debug, Eq, PartialEq)]
pub struct RepoPattern {
    /// The workspace name
    pub workspace: Option<String>,

    /// The remote name
    pub remote: Option<String>,

    /// The repo parent path
    pub path: Option<String>,

    /// The repo name
    pub name: String,
}

impl RepoPattern {
    pub fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
        match Regex::new(r"^([^/]+:)?([^/]+)?(.*/)?([^/])+$")?.captures(path) {
            Some(captures) => Ok(Self {
                workspace: captures
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .map(|s| s[..s.len() - 1].to_string()),
                remote: captures.get(2).map(|m| m.as_str().to_string()),
                path: captures.get(3).map(|m| m.as_str().to_string()),
                name: captures.get(4).unwrap().as_str().to_string(),
            }),
            None => bail!("Invalid repository pattern"),
        }
    }

    pub fn path(&self) -> String {
        match &self.path {
            Some(path) => format!("{}/{}", path, self.name),
            None => self.name.clone(),
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
            RepoPattern::parse("workspace12:remote1/abc/123")?,
            RepoPattern {
                workspace: Some("workspace12".to_string()),
                remote: Some("remote1".to_string()),
                path: Some("abc".to_string()),
                name: "123".to_string()
            }
        );
        assert_eq!(
            RepoPattern::parse("123")?,
            RepoPattern {
                workspace: None,
                remote: None,
                path: None,
                name: "123".to_string()
            }
        );

        Ok(())
    }
}
