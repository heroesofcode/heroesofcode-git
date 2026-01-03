use crate::cli_output::CliOutput;
use crate::network::Network;
use crate::utils::Utils;
use colored::Colorize;
use console::Term;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestItems {
	/// Array of all open pull requests
	pub items: Vec<PullRequestResponse>,
	pub total_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestResponse {
	/// Web URL of the repository on GitHub
	pub html_url: String,
	/// PR title
	pub title: String,
	/// User information who opened the PR
	pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
	/// Username of the user who opened the PR
	pub login: String,
}

pub struct PullRequests;

impl PullRequests {
	pub async fn pull_requests_open() -> Result<(), reqwest::Error> {
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

		let result: PullRequestItems = network.get_json(&url).await?;
		Ok(result)
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
