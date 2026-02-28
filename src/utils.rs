use comfy_table::Table;

/// Utility helpers for CLI output and formatting.
pub struct Utils;

impl Utils {
	/// Displays a formatted table in the terminal.
	pub fn table<T, I, F>(headers: &[&str], items: I, mut row: F)
	where
		I: IntoIterator<Item = T>,
		F: FnMut(T) -> Vec<String>,
	{
		let mut table = Table::new();
		table.set_header(headers.iter().map(|s| s.to_string()));

		for item in items {
			table.add_row(row(item));
		}

		println!("{table}");
	}
}
