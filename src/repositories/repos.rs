use serde::{Deserialize, Serialize};

use crate::github::GitHubClient;

/// GitHub repository response model
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoResponse {
	pub name: String,
	pub html_url: String,
	pub archived: bool,
	pub language: Option<String>,
}

/// Fetches repository data from the GitHub API
pub struct RepoRepository<C: GitHubClient> {
	client: C,
}

impl<C: GitHubClient> RepoRepository<C> {
	pub fn new(client: C) -> Self {
		Self { client }
	}

	/// Returns all non-archived repositories for the org
	pub async fn fetch(&self) -> Result<Vec<RepoResponse>, reqwest::Error> {
		let url = format!("{}/orgs/heroesofcode/repos", self.client.base_url());
		let repos: Vec<RepoResponse> = self.client.get_json(&url).await?;
		Ok(repos.into_iter().filter(|r| !r.archived).collect())
	}
}
