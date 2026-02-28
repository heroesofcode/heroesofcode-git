use colored::Colorize;
use console::Term;

/// Centralizes all CLI terminal output helpers
pub struct CliOutput;

impl CliOutput {
	pub fn success(term: &Term, message: &str) {
		term
			.write_line(&format!("{} {}", "✓".green(), message))
			.ok();
	}

	pub fn error(term: &Term, message: &str) {
		term.write_line(&format!("{} {}", "˟".red(), message)).ok();
	}

	pub fn loading(term: &Term, message: &str) {
		term.write_line(&format!("🔥 {message}...")).ok();
	}

	pub fn clear_last(term: &Term) {
		term.clear_last_lines(1).ok();
	}
}
