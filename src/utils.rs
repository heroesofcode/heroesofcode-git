use comfy_table::Table;

/// Utility helpers for CLI output and formatting.
pub struct Utils;

impl Utils {
	/// Displays a formatted table in the terminal.
	///
	/// This is a generic helper that builds a table from:
	/// - a list of column headers
	/// - an iterable collection of items
	/// - a closure that maps each item into a table row
	///
	/// The function is intentionally generic so it can be reused
	/// for different data types (e.g. repositories, pull requests,
	/// issues, or any other list-based data).
	///
	/// # Type Parameters
	///
	/// * `T` - The type of each item in the collection.
	/// * `I` - An iterable type that yields items of type `T`.
	/// * `F` - A closure that converts an item of type `T`
	///         into a `Vec<String>` representing a table row.
	///
	/// # Parameters
	///
	/// * `headers` - A slice of column titles. Each entry represents
	///   a column header in the table.
	/// * `items` - Any iterable collection of items to be displayed
	///   (e.g. a slice, a vector, or an iterator).
	/// * `row` - A closure that receives one item and returns
	///   a vector of strings, one per column.
	///
	/// # Example
	///
	/// ```rust
	/// use hoc::utils::Utils;
	///
	/// #[derive(Clone)]
	/// struct Repo { name: String, html_url: String }
	/// let repos = vec![
	///     Repo { name: "Example".to_string(), html_url: "https://example.com".to_string() },
	/// ];
	/// Utils::table(
	///     &["Name", "URL"],
	///     repos.iter().cloned(),
	///     |repo| vec![repo.name, repo.html_url],
	/// );
	/// ```
	///
	/// # Notes
	///
	/// - The table owns its data, so the closure must return owned `String`s.
	/// - This function is intended for read-only display purposes.
	/// - The number of columns is defined by the headers and row data.
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
