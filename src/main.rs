use hoc::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
	Cli::start().await?;
	Ok(())
}
