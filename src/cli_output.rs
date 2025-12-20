use colored::Colorize;
use console::Term;

/// Centralizes all CLI terminal output helpers
/// Responsible for printing success, error and info messages
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
}
