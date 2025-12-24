use console::Term;
use demand::{DemandOption, MultiSelect};
use std::{
	fs,
	path::Path,
	process::{Command, Stdio},
};

use crate::{
	cli_output::CliOutput,
	repos::{RepoResponse, Repos},
};

/// Handles repository cloning flow
pub struct Clone;

impl Clone {
	/// Fetches repositories and starts the interactive clone flow
	pub async fn clone_repos() -> Result<(), reqwest::Error> {
		println!();
		let term = Term::stdout();
		term.write_line("🔥 searching all repositories...").ok();

		match Repos::response().await {
			Ok(repos) => {
				term.clear_last_lines(1).ok();
				CliOutput::success(&term, &format!("repositories found"));
				Self::multi_select_validation(repos, &term);
				println!();

				Ok(())
			}
			Err(e) => {
				term.clear_last_lines(1).ok();
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
			if let Err(e) = Self::clone_repo(&url) {
				CliOutput::error(term, &format!("cloning {url}: {e}"));
				println!();
			} else {
				CliOutput::success(term, &format!("cloned {url}"));
			}
		}
	}

	/// Clones a repository into the local organization directory
	fn clone_repo(url: &str) -> Result<(), String> {
		let base = Path::new("heroesofcode");
		fs::create_dir_all(base).map_err(|e| e.to_string())?;

		let name = url.rsplit('/').next().ok_or("invalid url")?;
		let full_url = format!("{url}.git");

		if base.join(name).exists() {
			return Err("repository already exists".into());
		}

		Command::new("git")
			.args(["clone", &full_url])
			.current_dir(base)
			.stdout(Stdio::null())
			.stderr(Stdio::null())
			.status()
			.map_err(|e| e.to_string())?
			.success()
			.then_some(())
			.ok_or("git clone failed".into())
	}
}
