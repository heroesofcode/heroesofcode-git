use crate::cli_output::CliOutput;
use crate::network::Network;
use crate::utils::Utils;
use colored::Colorize;
use console::Term;
use serde::{Deserialize, Serialize};

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

pub struct PullRequests;

impl PullRequests {
	/// Fetch open pull requests (first page of paginated results)
	pub async fn list_open() -> Result<(), reqwest::Error> {
		match Self::response().await {
			Ok(result) => {
				println!();
				println!(
					"🔥 {} {}",
					"Total Pull Requests:".blue(),
					result.total_count
				);
				
				println!();

				Self::show_table(&result.items);

				let shown = result.items.len();
				if shown < result.total_count {
					println!(
						"{} {} of {} pull requests shown (paginated, truncated by GitHub API)",
						"⚠️".yellow(),
						shown,
						result.total_count
					);
				}
				Ok(())
			}
			Err(e) => {
				let term = Term::stdout();
				CliOutput::error(&term, &format!("fetching pull requests: {e}"));
				Err(e)
			}
		}
	}

	async fn response() -> Result<PullRequestItems, reqwest::Error> {
		let network = Network::new();
		let url = format!(
			"{}/search/issues?q=org:heroesofcode+type:pr+state:open",
			network.base_url()
		);

		network.get_json(&url).await
	}

	/// Renders pull requests data in a terminal table
	fn show_table(prs: &[PullRequestResponse]) {
		Utils::table(&["User", "Title", "URL"], prs, |item| {
			vec![
				item.user.login.clone(),
				item.title.clone(),
				item.html_url.clone(),
			]
		});
	}
}
