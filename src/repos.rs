use console::Term;
use serde::{Deserialize, Serialize};

use crate::utils::Utils;
use crate::{cli_output::CliOutput, network::Network};

/// GitHub repository response model
#[derive(Debug, Serialize, Deserialize)]
pub struct RepoResponse {
	/// Repository name
	pub name: String,
	/// Web URL of the repository on GitHub
	pub html_url: String,
}

/// Repository operations handler
pub struct Repos;

impl Repos {
	/// Fetches and displays all repositories in a table
	pub async fn list_all() -> Result<(), reqwest::Error> {
		println!();
		let term = Term::stdout();
		term.write_line("🔥 searching all repositories...").ok();

		match Self::response().await {
			Ok(result) => {
				term.clear_last_lines(1).ok();
				CliOutput::success(&term, "repositories found");
				println!();
				Self::show_table(&result);

				Ok(())
			}
			Err(e) => {
				term.clear_last_lines(1).ok();
				CliOutput::error(&term, &format!("listing repositories: {e}"));

				Err(e)
			}
		}
	}

	/// Requests the repository list from the API
	pub async fn response() -> Result<Vec<RepoResponse>, reqwest::Error> {
		let network = Network::new();
		let url = format!("{}/orgs/heroesofcode/repos", network.base_url());
		let result: Vec<RepoResponse> = network.get_json(&url).await?;
		Ok(result)
	}

	/// Renders repository data in a terminal table
	fn show_table(repos: &[RepoResponse]) {
		Utils::table(&["Repositories", "URL"], repos, |item| {
			vec![item.name.to_string(), item.html_url.to_string()]
		});
	}
}
