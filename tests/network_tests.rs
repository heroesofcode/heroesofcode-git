use httpmock::Method::GET;
use httpmock::MockServer;
use serde_json;

use hoc::network::Network;
use hoc::pull_requests::PullRequestItems;
use hoc::repos::RepoResponse;

#[tokio::test]
async fn test_get_json() {
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
}

#[tokio::test]
async fn test_get_pull_requests() {
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

	assert!(result.is_ok());
	let prs = result.unwrap();
	assert_eq!(prs.total_count, 2);
	assert_eq!(prs.items.len(), 2);
	assert_eq!(prs.items[0].title, "Test PR 1");
	assert_eq!(prs.items[0].user.login, "testuser1");
}
