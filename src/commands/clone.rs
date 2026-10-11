use std::{
	fs,
	path::{Path, PathBuf},
	process,
	sync::LazyLock,
};

use console::Term;
use demand::{DemandOption, MultiSelect, Theme};
use xx::git::{self, CloneOptions};

use hoc_github::{
	GitHubClient,
	repositories::repos::{RepoRepository, RepoResponse},
};
use hoc_ui::output::Output;

static DRACULA_THEME: LazyLock<Theme> = LazyLock::new(Theme::dracula);

pub struct CloneCommand;

impl CloneCommand {
	pub async fn execute<C: GitHubClient>(
		client: C,
		is_clone_all: bool,
		username: Option<&str>,
	) -> Result<(), reqwest::Error> {
		println!();
		let term = Term::stdout();
		let repo = RepoRepository::new(client);

		let result = match username {
			Some(username) => {
				Output::loading(
					&term,
					&format!("searching {username}'s public repositories"),
				);
				repo.fetch_user(username).await
			}
			None => {
				Output::loading(&term, "searching all repositories");
				repo.fetch().await
			}
		};
		Output::clear_last(&term);

		match result {
			Ok(repos) => {
				Output::success(&term, "repositories found");

				let cloned = if is_clone_all {
					clone_all(repos.iter(), &term)
				} else {
					interactive_select(&repos, &term)
				};

				println!();
				enter_cloned_dir(&cloned, &term);
				Ok(())
			}
			Err(e) => {
				Output::error(&term, &format!("listing repositories: {e}"));
				Err(e)
			}
		}
	}
}

/// Displays a multi-select prompt for repository selection.
/// Returns the paths of the repositories that were cloned (empty when the user cancelled).
fn interactive_select(repos: &[RepoResponse], term: &Term) -> Vec<PathBuf> {
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
			.description("Select languages to filter repositories (none = show all)")
			.theme(&DRACULA_THEME);

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
				return Vec::new();
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
		return Vec::new();
	}

	let mut multi_select = MultiSelect::new("Repositories")
		.description("Select the repositories you want to clone")
		.min(1)
		.filterable(true)
		.theme(&DRACULA_THEME);

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
			return Vec::new();
		}
	};

	if selected.contains(&CLONE_ALL_VALUE) {
		clone_all(filtered_repos.iter().copied(), term)
	} else {
		selected
			.into_iter()
			.filter_map(|url| {
				let repo = filtered_repos.iter().find(|r| r.html_url == url)?;
				handle_clone_result(url, &repo.name, repo.language.as_deref(), term)
			})
			.collect()
	}
}

/// Clones each repository in order and returns the paths that were cloned
fn clone_all<'a>(repos: impl IntoIterator<Item = &'a RepoResponse>, term: &Term) -> Vec<PathBuf> {
	repos
		.into_iter()
		.filter_map(|repo| {
			handle_clone_result(&repo.html_url, &repo.name, repo.language.as_deref(), term)
		})
		.collect()
}

/// Clones a repository and outputs the result
fn handle_clone_result(
	url: &str,
	name: &str,
	language: Option<&str>,
	term: &Term,
) -> Option<PathBuf> {
	match clone_repo(url, language) {
		Ok(dest) => {
			Output::success(term, &format!("{name} cloned"));
			Some(dest)
		}
		Err(e) => {
			Output::error(term, &format!("{name} {e}"));
			None
		}
	}
}

/// Opens a shell inside the cloned repository, or inside the heroesofcode
/// folder when more than one repository was cloned
fn enter_cloned_dir(cloned: &[PathBuf], term: &Term) {
	let dir = match cloned {
		[] => return,
		[single] => single.clone(),
		_ => match heroesofcode_dir() {
			Ok(dir) => dir,
			Err(e) => {
				Output::error(term, &e);
				return;
			}
		},
	};

	open_shell(&dir, term);
}

/// Spawns the user's shell in `dir`; exiting it returns to the previous shell
fn open_shell(dir: &Path, term: &Term) {
	#[cfg(windows)]
	let shell = std::env::var("COMSPEC").unwrap_or_else(|_| "cmd".into());
	#[cfg(not(windows))]
	let shell = std::env::var("SHELL").unwrap_or_else(|_| "sh".into());

	Output::success(term, &format!("entering {}", dir.display()));

	if let Err(e) = process::Command::new(shell).current_dir(dir).status() {
		Output::error(term, &format!("opening shell: {e}"));
	}
}

/// Returns the heroesofcode folder in the user's Documents directory
fn heroesofcode_dir() -> Result<PathBuf, String> {
	dirs::document_dir()
		.map(|dir| dir.join("heroesofcode"))
		.ok_or_else(|| "Could not find Documents".into())
}

/// Clones a repository into the heroesofcode/<language> folder in the user's Documents directory
fn clone_repo(url: &str, language: Option<&str>) -> Result<PathBuf, String> {
	let heroesofcode_dir = heroesofcode_dir()?;

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
