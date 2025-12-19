use httpmock::Method::GET;
use httpmock::MockServer;
use serde_json;

use heroesofcode_git::network::Network;
use heroesofcode_git::repos::RepoResponse;

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
