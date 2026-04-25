use console::Term;

use crate::{
	github::GitHubClient,
	output::Output,
	repositories::repos::{RepoRepository, RepoResponse},
	utils::Utils,
};

pub struct ListReposCommand;

impl ListReposCommand {
	pub async fn execute<C: GitHubClient>(client: C) -> Result<(), reqwest::Error> {
		println!();
		let term = Term::stdout();
		let repo = RepoRepository::new(client);

		Output::loading(&term, "searching all repositories");

		let result = repo.fetch().await;
		Output::clear_last(&term);

		match result {
			Ok(repos) => {
				Output::success(&term, "repositories found");
				println!();
				show_table(&repos);
				Ok(())
			}
			Err(e) => {
				Output::error(&term, &format!("listing repositories: {e}"));
				Err(e)
			}
		}
	}
}

fn show_table(repos: &[RepoResponse]) {
	Utils::table(&["Repositories", "URL"], repos, |item| {
		vec![item.name.clone(), item.html_url.clone()]
	});
}
