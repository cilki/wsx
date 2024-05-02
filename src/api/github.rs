use crate::api::Provider;
use crate::Provider;
use anyhow::Result;
use std::fmt;

pub struct GithubProvider {
    pub config: Provider,
}

impl fmt::Display for GithubProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Github",)
    }
}

impl Provider for GithubProvider {
    fn list_repo_paths(&self) -> Result<Vec<String>> {
        let mut paths: Vec<String> = Vec::new();

        /*loop {
            let q = Repositories::build_query(repositories::Variables {
                login: self.name.to_lowercase(),
                include_forks,
                after,
            });
            let res = ureq::post("https://api.github.com/graphql")
                .set("Authorization", format!("Bearer {}", github_token).as_str())
                .send_json(json!(&q))?;
            let response_data: Response<repositories::ResponseData> =
                serde_json::from_value(res.into_json()?)?;
            let response_repositories = response_data
                .data
                .unwrap_or_else(|| panic!("Invalid response from GitHub for user {}", self.name))
                .repository_owner
                .unwrap_or_else(|| panic!("Invalid response from GitHub for user {}", self.name))
                .repositories;

            repositories.extend(
                response_repositories
                    .nodes
                    .unwrap()
                    .iter()
                    .map(|r| r.as_ref().unwrap())
                    .filter(|r| !r.is_archived)
                    .map(|repo| self.parse_repo(&self.path, repo)),
            );

            if !response_repositories.page_info.has_next_page {
                break;
            }
            after = response_repositories.page_info.end_cursor;
        }*/

        Ok(paths)
    }
}
