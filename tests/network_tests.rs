use httpmock::Method::GET;
use httpmock::MockServer;

use hoc::network::Network;
use hoc::pull_requests::PullRequestItems;
use hoc::repos::RepoResponse;

#[tokio::test]
async fn test_get_json_success() {
	let server = MockServer::start();
	server.mock(|when, then| {
		when.method(GET).path("/orgs/heroesofcode/repos");
		then.status(200).json_body_obj(&vec![
			serde_json::json!({
					"name": "ViewState",
					"html_url": "https://github.com/heroesofcode/ViewState"
			}),
			serde_json::json!({
					"name": "DataLife",
					"html_url": "https://github.com/heroesofcode/DataLife"
			}),
		]);
	});

	let network = Network::new();
	let url = &format!("{}/orgs/heroesofcode/repos", server.base_url());
	let result = network.get_json::<Vec<RepoResponse>>(url).await;
	assert!(result.is_ok());
	let repos = result.unwrap();
	assert_eq!(repos.len(), 2);
	assert_eq!(repos[0].name, "ViewState");
	assert_eq!(repos[1].name, "DataLife");
}

#[tokio::test]
async fn test_get_json_error_status() {
	let server = MockServer::start();
	server.mock(|when, then| {
		when.method(GET).path("/error");
		then.status(404);
	});

	let network = Network::new();
	let url = &format!("{}/error", server.base_url());
	let result = network.get_json::<Vec<RepoResponse>>(url).await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_get_json_invalid_json() {
	let server = MockServer::start();
	server.mock(|when, then| {
		when.method(GET).path("/invalid");
		then.status(200).body("not json");
	});

	let network = Network::new();
	let url = &format!("{}/invalid", server.base_url());
	let result = network.get_json::<Vec<RepoResponse>>(url).await;
	assert!(result.is_err());
}

#[tokio::test]
async fn test_get_pull_requests_success() {
	let server = MockServer::start();
	server.mock(|when, then| {
		when.method(GET).path("/search/issues");
		then.status(200).json_body_obj(&serde_json::json!({
			"total_count": 2,
			"items": [
				{
					"html_url": "https://github.com/heroesofcode/test-repo/pull/1",
					"title": "Test PR 1",
					"user": {
						"login": "testuser1"
					}
				},
				{
					"html_url": "https://github.com/heroesofcode/test-repo/pull/2",
					"title": "Test PR 2",
					"user": {
						"login": "testuser2"
					}
				}
			]
		}));
	});

	let network = Network::new();
	let url = &format!(
		"{}/search/issues?q=org:heroesofcode+type:pr+state:open",
		server.base_url()
	);
	let result = network.get_json::<PullRequestItems>(url).await;

	let prs = result.expect("Failed to get pull requests");
	assert_eq!(prs.total_count, 2);
	assert_eq!(prs.items.len(), 2);
	assert_eq!(prs.items[0].title, "Test PR 1");
	assert_eq!(prs.items[0].user.login, "testuser1");
	assert_eq!(prs.items[1].title, "Test PR 2");
	assert_eq!(prs.items[1].user.login, "testuser2");
}

#[tokio::test]
async fn test_get_pull_requests_empty() {
	let server = MockServer::start();
	server.mock(|when, then| {
		when.method(GET).path("/search/issues");
		then.status(200).json_body_obj(&serde_json::json!({
			"total_count": 0,
			"items": []
		}));
	});

	let network = Network::new();
	let url = &format!("{}/search/issues", server.base_url());
	let result = network.get_json::<PullRequestItems>(url).await;

	let prs = result.expect("Failed to get pull requests");
	assert_eq!(prs.total_count, 0);
	assert_eq!(prs.items.len(), 0);
}
