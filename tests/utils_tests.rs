use hoc::utils::Utils;

#[test]
fn test_table_with_single_row() {
	struct Item {
		name: String,
		value: String,
	}

	let items = vec![Item {
		name: "test".to_string(),
		value: "123".to_string(),
	}];

	Utils::table(&["Name", "Value"], items, |item| {
		vec![item.name, item.value]
	});
}

#[test]
fn test_table_with_multiple_rows() {
	struct Item {
		id: u32,
		name: String,
	}

	let items = vec![
		Item {
			id: 1,
			name: "first".to_string(),
		},
		Item {
			id: 2,
			name: "second".to_string(),
		},
		Item {
			id: 3,
			name: "third".to_string(),
		},
	];

	Utils::table(&["ID", "Name"], items, |item| {
		vec![item.id.to_string(), item.name]
	});
}

#[test]
fn test_table_with_empty_data() {
	let items: Vec<(String, String)> = vec![];

	Utils::table(&["Column1", "Column2"], items, |item| vec![item.0, item.1]);
}

#[test]
fn test_table_with_special_characters() {
	let items = vec![
		("Name with spaces", "Value with \"quotes\""),
		("Unicode: 🔥", "Special: &<>"),
	];

	Utils::table(&["Key", "Value"], items, |item| {
		vec![item.0.to_string(), item.1.to_string()]
	});
}

#[test]
fn test_table_with_many_columns() {
	let items = vec![("A", "B", "C", "D", "E")];

	Utils::table(&["Col1", "Col2", "Col3", "Col4", "Col5"], items, |item| {
		vec![
			item.0.to_string(),
			item.1.to_string(),
			item.2.to_string(),
			item.3.to_string(),
			item.4.to_string(),
		]
	});
}
