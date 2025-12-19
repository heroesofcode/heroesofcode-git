use demand::{DemandOption, MultiSelect};
use std::{fs, path::Path, process::Command};

use crate::repos::{RepoResponse, Repos};

/// Handles repository cloning flow
pub struct Clone;

impl Clone {
	/// Fetches repositories and starts the interactive clone flow
	pub async fn clone_repos() -> Result<(), reqwest::Error> {
		println!();
		println!("🔥 Searching all repositories");

		match Repos::response().await {
			Ok(repos) => {
				println!("✅ All repositories with success");
				println!();
				Self::multi_select_validation(repos);
				println!();

				Ok(())
			}
			Err(error) => {
				eprintln!("❌ Error listing repositories: {}", error);
				Err(error)
			}
		}
	}

	/// Displays a multi-select prompt for repository selection
	fn multi_select_validation(repos: Vec<RepoResponse>) {
		let mut multi_select = MultiSelect::new("Repositories")
			.description("Select the repositories you want to clone")
			.min(1)
			.filterable(true);

		for repo in repos {
			multi_select = multi_select.option(DemandOption::new(repo.html_url).label(&repo.name));
		}

		let selected = multi_select.run().expect("error running multi select");

		for url in selected {
			if let Err(error) = Self::clone_repo(&url) {
				println!("❌ Error cloning {url}: {error}");
				println!();
			} else {
				println!("✅ Success in cloning {url}");
				println!();
			}
		}
	}

	/// Clones a repository into the local organization directory
	fn clone_repo(url: &str) -> Result<(), String> {
		let base = Path::new("heroesofcode");
		fs::create_dir_all(base).map_err(|error| error.to_string())?;

		let name = url.rsplit('/').next().ok_or("invalid url")?;
		let full_url = format!("{url}.git");

		if base.join(name).exists() {
			return Err("Repository already exists".into());
		}

		Command::new("git")
			.args(["clone", &full_url])
			.current_dir(base)
			.status()
			.map_err(|e| e.to_string())?
			.success()
			.then_some(())
			.ok_or("git clone failed".into())
	}
}
