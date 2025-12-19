use clap::{Parser, Subcommand};

use heroesofcode_git::clone::Clone;
use heroesofcode_git::repos::Repos;

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
	/// Clone some project
	Clone,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	let args = Args::parse();

	match args.command {
		Some(Command::Repos) => {
			Repos::list_all().await?;
		}
		Some(Command::Clone) => {
			Clone::clone_repos().await?;
		}
		None => {
			println!("Run --help");
		}
	}

	Ok(())
}
