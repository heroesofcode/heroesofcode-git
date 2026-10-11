# CLAUDE.md

## Project Overview

**heroesofcode-git** is a Rust CLI tool (`hoc`) for Heroes of Code maintainers to manage and clone repositories from the Heroes of Code GitHub organization.

## Commands

```bash
# Build
cargo build --workspace  # debug build
cargo build --release    # release build

# Run
cargo run                # run the CLI

# Test
cargo test --workspace

# Lint & Format
cargo clippy --workspace --all-targets --all-features
cargo fmt --all -- --check

# Via mise task runner
mise build
mise test
mise lint
mise fmt
mise check
mise release
mise doc
mise zizmor      # audit GitHub Actions workflows
```

## CLI Usage

```bash
hoc repos   # list all repositories
hoc clone   # clone repositories (interactive)
hoc all     # clone all repositories
hoc pr      # show open pull requests
hoc my      # clone joaolfp's public repositories (interactive)
```

## Architecture

Cargo workspace. The root package is the `hoc` binary; libraries live in `crates/`.

- `src/main.rs` — entry point, sets up tokio async runtime
- `src/cli.rs` — command router using `clap::Subcommand`
- `src/commands/` — `clone`, `list_repos`, `list_prs` command implementations
- `crates/hoc-github` — `GitHubClient` trait (`lib.rs`), `reqwest` wrapper `Network` (`client.rs`), and `repositories/` (`RepoRepository`, `PrRepository` + response models)
- `crates/hoc-ui` — terminal output helpers (`output.rs`) and generic `comfy-table` rendering (`utils.rs`)

Shared dependency versions are declared once in `[workspace.dependencies]` of the root `Cargo.toml`.

## Testing

Integration tests live in each crate's `tests/` (e.g. `crates/hoc-github/tests/`) and use `httpmock` to mock HTTP servers. Unit tests (deserialization) live in module files. Test naming convention: `test_<feature>_<scenario>`.

## Code Conventions

- Rust Edition 2024, toolchain pinned to 1.94.0 (`rust-toolchain.toml`)
- Async-first: tokio runtime, `.await` throughout
- Error handling: `Result<(), reqwest::Error>` for fallible ops
- Formatting: enforced by `.rustfmt.toml` (hard tabs, `group_imports = "StdExternalCrate"`)
- Conventional commits required

## CI/CD

- **CI.yml** — runs on PRs: build + test via `mise`
- **autofix.yml** — runs clippy fixes + rustfmt, auto-commits on PRs
- **Release.yml** — creates git tag when commit message contains "Prepare version to"
- **zizmor.yml** — static security analysis of GitHub Actions workflows (results in Code Scanning)
- Renovate auto-merges dependency updates on green CI
