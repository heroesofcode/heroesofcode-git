use console::Term;
use demand::{DemandOption, MultiSelect};
use std::fs;

use xx::git::{self, CloneOptions};

use crate::{
	cli_output::CliOutput,
	repos::{RepoResponse, Repos},
};

/// Handles repository cloning flow
pub struct Clone;

impl Clone {
	/// Fetches repositories and initiates clone flow (interactive or clone-all mode)
	pub async fn clone_repos(is_clone_all: bool) -> Result<(), reqwest::Error> {
		println!();
		let term = Term::stdout();
		CliOutput::loading(&term, "searching all repositories");

		match Repos::response().await {
			Ok(repos) => {
				CliOutput::clear_last(&term);
				CliOutput::success(&term, "repositories found");

				if is_clone_all {
					Self::clone_all_repos(repos, &term);
				} else {
					Self::multi_select_validation(repos, &term);
				}
				println!();
				Ok(())
			}
			Err(e) => {
				CliOutput::clear_last(&term);
				CliOutput::error(&term, &format!("listing repositories: {e}"));
				Err(e)
			}
		}
	}

	/// Displays a multi-select prompt for repository selection
	fn multi_select_validation(repos: Vec<RepoResponse>, term: &Term) {
		let mut multi_select = MultiSelect::new("Repositories")
			.description("Select the repositories you want to clone")
			.min(1)
			.filterable(true);

		for repo in repos {
			multi_select = multi_select.option(DemandOption::new(repo.html_url).label(&repo.name));
		}

		let selected = match multi_select.run() {
			Ok(selection) => selection,
			Err(e) => {
				let message = if e.kind() == std::io::ErrorKind::Interrupted {
					"Operation interrupted by user"
				} else {
					"Error selecting options"
				};
				CliOutput::error(term, message);
				return;
			}
		};

		for url in selected {
			Self::handle_clone_result(&url, term);
		}
	}

	/// Clones all repositories without user interaction and outputs results
	fn clone_all_repos(repos: Vec<RepoResponse>, term: &Term) {
		for repo in repos {
			Self::handle_clone_result(&repo.html_url, term);
		}
	}

	/// Clones a repository and outputs the result (success or error message)
	fn handle_clone_result(url: &str, term: &Term) {
		match Self::clone_repo(url) {
			Ok(()) => CliOutput::success(
				term,
				&format!("cloned {url}. You can find it in the 'heroesofcode' folder on your Desktop."),
			),
			Err(e) => CliOutput::error(term, &format!("cloning {url}: {e}")),
		}
	}

	/// Clones a repository into the heroesofcode folder on the user's Desktop
	fn clone_repo(url: &str) -> Result<(), String> {
		let base = dirs::desktop_dir()
			.ok_or("Could not find Desktop")?
			.join("heroesofcode");

		fs::create_dir_all(&base).map_err(|e| e.to_string())?;

		let name = url.rsplit('/').next().ok_or("invalid url")?;
		let dest = base.join(name);

		if dest.exists() {
			return Err("repository already exists".into());
		}

		let full_url = if url.ends_with(".git") {
			url.to_string()
		} else {
			format!("{url}.git")
		};

		git::clone(&full_url, &dest, &CloneOptions::default())
			.map(|_| ())
			.map_err(|e| e.to_string())
	}
}
