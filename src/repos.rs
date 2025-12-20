use comfy_table::Table;
use console::Term;
use serde::{Deserialize, Serialize};

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
			Err(error) => {
				term.clear_last_lines(1).ok();
				CliOutput::error(&term, &format!("listing repositories: {error}"));

				Err(error)
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
		let mut table = Table::new();

		let titles = vec!["Repositories", "URL"];
		table.set_header(titles);

		for repo in repos {
			let rows = vec![repo.name.to_string(), repo.html_url.to_string()];
			table.add_row(rows);
		}

		println!("{table}");
	}
}
