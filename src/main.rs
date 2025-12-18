use clap::{Parser, Subcommand};

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
			Repos::response().await?;
		}
		Some(Command::Clone) => {
			println!("Clone some project");
		}
		None => {
			println!("Run --help");
		}
	}

	Ok(())
}
