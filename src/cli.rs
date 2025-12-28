use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::clone::Clone;
use crate::repos::Repos;

#[derive(Parser)]
struct Args {
	/// Optional subcommand that defines an alternative executions flow
	#[command[subcommand]]
	command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
	/// List all repositories
	Repos,
	/// Clone repositories
	Clone,
	/// Clone all repositories
	All,
}

pub struct Cli;

impl Cli {
	/// Function to initialize the tool
	pub async fn start() -> Result<(), reqwest::Error> {
		let args = Args::parse();

		match args.command {
			Some(Command::Repos) => {
				Repos::list_all().await?;
			}
			Some(Command::Clone) => {
				Clone::clone_repos(false).await?;
			}
			Some(Command::All) => {
				Clone::clone_repos(true).await?;
			}
			None => {
				println!("Run {}", "heroesofcode --help".blue());
			}
		}

		Ok(())
	}
}
