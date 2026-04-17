use serde::{Deserialize, Serialize};

use crate::github::GitHubClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestItems {
	pub items: Vec<PullRequestResponse>,
	pub total_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestResponse {
	pub html_url: String,
	pub title: String,
	pub user: PullRequestUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestUser {
	pub login: String,
}

/// Fetches pull request data from the GitHub API
pub struct PrRepository<C: GitHubClient> {
	client: C,
}

impl<C: GitHubClient> PrRepository<C> {
	pub fn new(client: C) -> Self {
		Self { client }
	}

	/// Returns open pull requests across the org
	pub async fn fetch(&self) -> Result<PullRequestItems, reqwest::Error> {
		let url = format!(
			"{}/search/issues?q=org:heroesofcode+type:pr+state:open",
			self.client.base_url()
		);
		self.client.get_json(&url).await
	}
}
