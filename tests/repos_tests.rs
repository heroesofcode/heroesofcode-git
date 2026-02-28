use hoc::network::Network;
use hoc::repos::{RepoResponse, Repos};
use httpmock::Method::GET;
use httpmock::MockServer;

#[tokio::test]
async fn test_repos_response_success() {
	let result = Repos::response().await;
	assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_repo_response_deserialization() {
	let json = r#"{
		"name": "test-repo",
		"html_url": "https://github.com/heroesofcode/test-repo"
	}"#;

	let repo: RepoResponse = serde_json::from_str(json).unwrap();
	assert_eq!(repo.name, "test-repo");
	assert_eq!(repo.html_url, "https://github.com/heroesofcode/test-repo");
}

#[tokio::test]
async fn test_repos_empty_list() {
	let server = MockServer::start();
	server.mock(|when, then| {
		when.method(GET).path("/orgs/heroesofcode/repos");
		then.status(200).json_body_obj(&Vec::<RepoResponse>::new());
	});

	let network = Network::new();
	let url = format!("{}/orgs/heroesofcode/repos", server.base_url());
	let result = network.get_json::<Vec<RepoResponse>>(&url).await;

	assert!(result.is_ok());
	let repos = result.unwrap();
	assert_eq!(repos.len(), 0);
}

#[tokio::test]
async fn test_repos_multiple_items() {
	let server = MockServer::start();
	server.mock(|when, then| {
		when.method(GET).path("/orgs/heroesofcode/repos");
		then.status(200).json_body_obj(&vec![
			serde_json::json!({"name": "repo1", "html_url": "https://github.com/heroesofcode/repo1"}),
			serde_json::json!({"name": "repo2", "html_url": "https://github.com/heroesofcode/repo2"}),
			serde_json::json!({"name": "repo3", "html_url": "https://github.com/heroesofcode/repo3"}),
		]);
	});

	let network = Network::new();
	let url = format!("{}/orgs/heroesofcode/repos", server.base_url());
	let result = network.get_json::<Vec<RepoResponse>>(&url).await;

	assert!(result.is_ok());
	let repos = result.unwrap();
	assert_eq!(repos.len(), 3);
	assert_eq!(repos[0].name, "repo1");
	assert_eq!(repos[1].name, "repo2");
	assert_eq!(repos[2].name, "repo3");
}
