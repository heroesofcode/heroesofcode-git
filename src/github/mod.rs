pub mod client;

use serde::de::DeserializeOwned;

/// Abstraction over HTTP access to the GitHub API.
/// Implement this trait to swap the real client for a mock in tests.
#[allow(async_fn_in_trait)]
pub trait GitHubClient {
	async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T, reqwest::Error>;
	fn base_url(&self) -> &str;
}
