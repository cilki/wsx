use regex::Regex;
use simple_error::bail;
use std::error::Error;

pub mod drop;

/// Represents a pattern that matches one or more repositories. It has the
/// format: [workspace]:[path].
#[derive(Debug, Eq, PartialEq)]
pub struct RepoPattern {
    /// The workspace name
    pub workspace: Option<String>,

    /// The repo path
    pub path: String,
}

impl RepoPattern {
    pub fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
        match Regex::new(r"^([^/]+:)?(.*)$")?.captures(path) {
            Some(captures) => Ok(Self {
                workspace: captures
                    .get(1)
                    .map(|m| m.as_str().to_string())
                    .map(|s| s[..s.len() - 1].to_string()),
                path: captures.get(2).unwrap().as_str().to_string(),
            }),
            None => bail!("Invalid repository path pattern"),
        }
    }

    pub fn maybe_provider(&self) -> Option<(String, String)> {
        let parts: Vec<&str> = self.path.splitn(1, "/").collect();
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
            RepoPattern::parse("workspace12:remote1/abc/123")?,
            RepoPattern {
                workspace: Some("workspace12".to_string()),
                path: "remote1/abc/123".to_string()
            }
        );
        assert_eq!(
            RepoPattern::parse("123")?,
            RepoPattern {
                workspace: None,
                path: "123".to_string()
            }
        );

        Ok(())
    }
}
