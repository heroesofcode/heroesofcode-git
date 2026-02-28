use hoc::pull_requests::{PullRequestItems, PullRequestResponse, PullRequestUser};

#[test]
fn test_pull_request_response_deserialization() {
	let json = r#"{
		"html_url": "https://github.com/heroesofcode/test/pull/1",
		"title": "Add feature",
		"user": {
			"login": "testuser"
		}
	}"#;

	let pr: PullRequestResponse = serde_json::from_str(json).unwrap();
	assert_eq!(pr.html_url, "https://github.com/heroesofcode/test/pull/1");
	assert_eq!(pr.title, "Add feature");
	assert_eq!(pr.user.login, "testuser");
}

#[test]
fn test_pull_request_items_deserialization() {
	let json = r#"{
		"total_count": 2,
		"items": [
			{
				"html_url": "https://github.com/heroesofcode/test/pull/1",
				"title": "PR 1",
				"user": {"login": "user1"}
			},
			{
				"html_url": "https://github.com/heroesofcode/test/pull/2",
				"title": "PR 2",
				"user": {"login": "user2"}
			}
		]
	}"#;

	let items: PullRequestItems = serde_json::from_str(json).unwrap();
	assert_eq!(items.total_count, 2);
	assert_eq!(items.items.len(), 2);
	assert_eq!(items.items[0].title, "PR 1");
	assert_eq!(items.items[1].user.login, "user2");
}

#[test]
fn test_pull_request_user_deserialization() {
	let json = r#"{"login": "johndoe"}"#;
	let user: PullRequestUser = serde_json::from_str(json).unwrap();
	assert_eq!(user.login, "johndoe");
}

#[test]
fn test_pull_request_items_empty() {
	let json = r#"{
		"total_count": 0,
		"items": []
	}"#;

	let items: PullRequestItems = serde_json::from_str(json).unwrap();
	assert_eq!(items.total_count, 0);
	assert_eq!(items.items.len(), 0);
}

#[test]
fn test_pull_request_with_special_characters() {
	let json = r#"{
		"html_url": "https://github.com/heroesofcode/test/pull/1",
		"title": "Fix: bug with \"quotes\" & special chars",
		"user": {"login": "user-name_123"}
	}"#;

	let pr: PullRequestResponse = serde_json::from_str(json).unwrap();
	assert_eq!(pr.title, "Fix: bug with \"quotes\" & special chars");
	assert_eq!(pr.user.login, "user-name_123");
}
