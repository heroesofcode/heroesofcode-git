use std::fs;

use console::Term;
use demand::{DemandOption, MultiSelect};
use xx::git::{self, CloneOptions};

use crate::{
	github::client::Network,
	output::Output,
	repositories::repos::{RepoRepository, RepoResponse},
};

pub struct CloneCommand;

impl CloneCommand {
	pub async fn execute(is_clone_all: bool) -> Result<(), reqwest::Error> {
		println!();
		let term = Term::stdout();
		let repo = RepoRepository::new(Network::new());

		Output::loading(&term, "searching all repositories");
		let result = repo.fetch().await;
		Output::clear_last(&term);

		match result {
			Ok(repos) => {
				Output::success(&term, "repositories found");

				if is_clone_all {
					clone_all(repos.iter(), &term);
				} else {
					interactive_select(repos, &term);
				}

				println!();
				Ok(())
			}
			Err(e) => {
				Output::error(&term, &format!("listing repositories: {e}"));
				Err(e)
			}
		}
	}
}

/// Displays a multi-select prompt for repository selection
fn interactive_select(repos: Vec<RepoResponse>, term: &Term) {
	let mut languages: Vec<&str> = repos
		.iter()
		.filter_map(|r| r.language.as_deref())
		.collect::<std::collections::HashSet<_>>()
		.into_iter()
		.collect();

	languages.sort();

	let selected_langs: Vec<&str> = if languages.is_empty() {
		Vec::new()
	} else {
		let mut lang_select = MultiSelect::new("Languages")
			.description("Select languages to filter repositories (none = show all)");

		for lang in &languages {
			lang_select = lang_select.option(DemandOption::new(*lang).label(lang));
		}

		match lang_select.run() {
			Ok(s) => s,
			Err(e) => {
				let message = if e.kind() == std::io::ErrorKind::Interrupted {
					"Operation interrupted by user"
				} else {
					"Error selecting languages"
				};

				Output::error(term, message);
				return;
			}
		}
	};

	let filtered_repos: Vec<&RepoResponse> = repos
		.iter()
		.filter(|r| {
			selected_langs.is_empty()
				|| r
					.language
					.as_deref()
					.is_some_and(|lang| selected_langs.contains(&lang))
		})
		.collect();

	const CLONE_ALL_VALUE: &str = "__clone_all__";

	if filtered_repos.is_empty() {
		let message = if repos.is_empty() {
			"no repositories available to clone"
		} else {
			"no repositories found matching the filter"
		};

		Output::error(term, message);
		return;
	}

	let mut multi_select = MultiSelect::new("Repositories")
		.description("Select the repositories you want to clone")
		.min(1)
		.filterable(true);

	if filtered_repos.len() > 1 {
		multi_select = multi_select.option(DemandOption::new(CLONE_ALL_VALUE).label("Clone All"));
	}

	for repo in &filtered_repos {
		multi_select = multi_select.option(DemandOption::new(repo.html_url.as_str()).label(&repo.name));
	}

	let selected = match multi_select.run() {
		Ok(selection) => selection,
		Err(e) => {
			let message = if e.kind() == std::io::ErrorKind::Interrupted {
				"Operation interrupted by user"
			} else {
				"Error selecting options"
			};

			Output::error(term, message);
			return;
		}
	};

	if selected.contains(&CLONE_ALL_VALUE) {
		clone_all(filtered_repos.iter().copied(), term);
	} else {
		for url in selected {
			let language = filtered_repos
				.iter()
				.find(|r| r.html_url == url)
				.and_then(|r| r.language.as_deref());

			handle_clone_result(url, language, term);
		}
	}
}

/// Clones each repository in order
fn clone_all<'a>(repos: impl IntoIterator<Item = &'a RepoResponse>, term: &Term) {
	for repo in repos {
		handle_clone_result(&repo.html_url, repo.language.as_deref(), term);
	}
}

/// Clones a repository and outputs the result
fn handle_clone_result(url: &str, language: Option<&str>, term: &Term) {
	match clone_repo(url, language) {
		Ok(path) => Output::success(term, &format!("cloned {url} → {}", path.display())),
		Err(e) => Output::error(term, &format!("cloning {url}: {e}")),
	}
}

/// Clones a repository into the heroesofcode/<language> folder on the user's Desktop
fn clone_repo(url: &str, language: Option<&str>) -> Result<std::path::PathBuf, String> {
	let heroesofcode_dir = dirs::desktop_dir()
		.ok_or("Could not find Desktop")?
		.join("heroesofcode");

	let base = match language {
		Some(lang) => heroesofcode_dir.join(lang.to_lowercase()),
		None => heroesofcode_dir,
	};

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
		.map(|_| dest)
		.map_err(|e| e.to_string())
}
