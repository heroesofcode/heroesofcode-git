use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct RepoResponse {
	name: String,
}

pub struct Repos;

impl Repos {
	pub async fn response() -> Result<(), reqwest::Error> {
		//let url = "https://api.github.com/orgs/heroesofcode/repos";
		let url = "http://localhost:3001/orgs/heroesofcode/repos";

		let response = reqwest::Client::new()
			.get(url)
			.header(reqwest::header::USER_AGENT, "MyInfo")
			.send()
			.await?;

		if response.status().is_success() {
			let result: Vec<RepoResponse> = response.json().await?;
			println!("{:?}", result);
			//Self::validation_response(&result)
		} else {
			println!("Request failed with status code: {}", response.status());
		}

		Ok(())
	}
}
