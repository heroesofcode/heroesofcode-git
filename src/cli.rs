use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::{
	commands::{clone::CloneCommand, list_prs::ListPrsCommand, list_repos::ListReposCommand},
	github::client::Network,
};

#[derive(Parser)]
struct Args {
	#[command(subcommand)]
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
	/// Show open pull requests
	Pr,
}

pub struct Cli;

impl Cli {
	pub async fn start() -> Result<(), reqwest::Error> {
		let args = Args::parse();
		let client = Network::new();

		match args.command {
			Some(Command::Repos) => ListReposCommand::execute(client).await,
			Some(Command::Clone) => CloneCommand::execute(client, false).await,
			Some(Command::All) => CloneCommand::execute(client, true).await,
			Some(Command::Pr) => ListPrsCommand::execute(client).await,
			None => {
				println!("Run {}", "heroesofcode --help".blue());
				Ok(())
			}
		}
	}
}
