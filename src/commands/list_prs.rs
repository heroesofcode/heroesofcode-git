use colored::Colorize;
use console::Term;

use crate::{
	github::client::Network,
	output::Output,
	repositories::pull_requests::{PrRepository, PullRequestResponse},
	utils::Utils,
};

pub struct ListPrsCommand;

impl ListPrsCommand {
	pub async fn execute() -> Result<(), reqwest::Error> {
		let term = Term::stdout();
		let repo = PrRepository::new(Network::new());

		match repo.fetch().await {
			Ok(result) => {
				println!();
				println!(
					"🔥 {} {}",
					"Total Pull Requests:".blue(),
					result.total_count
				);

				println!();
				show_table(&result.items);

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
				Output::error(&term, &format!("fetching pull requests: {e}"));
				Err(e)
			}
		}
	}
}

fn show_table(prs: &[PullRequestResponse]) {
	Utils::table(&["User", "Title", "URL"], prs, |item| {
		vec![
			item.user.login.clone(),
			item.title.clone(),
			item.html_url.clone(),
		]
	});
}
