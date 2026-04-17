use reqwest::{Client, header};
use serde::de::DeserializeOwned;

use super::GitHubClient;

/// Lightweight HTTP client wrapper using `reqwest`
pub struct Network {
	client: Client,
}

impl Default for Network {
	fn default() -> Self {
		Self::new()
	}
}

impl Network {
	/// Creates a reusable HTTP client with default headers
	pub fn new() -> Self {
		let mut headers = header::HeaderMap::new();
		headers.insert(
			header::USER_AGENT,
			header::HeaderValue::from_static("heroesofcode-git"),
		);

		let client = Client::builder()
			.default_headers(headers)
			.build()
			.expect("failed to build reqwest client");

		Self { client }
	}
}

impl GitHubClient for Network {
	async fn get_json<T>(&self, url: &str) -> Result<T, reqwest::Error>
	where
		T: DeserializeOwned,
	{
		self
			.client
			.get(url)
			.send()
			.await?
			.error_for_status()?
			.json()
			.await
	}

	fn base_url(&self) -> &str {
		if cfg!(debug_assertions) {
			"http://localhost:3001"
		} else {
			"https://api.github.com"
		}
	}
}
