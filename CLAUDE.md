# CLAUDE.md

## Project Overview

**heroesofcode-git** is a Rust CLI tool (`hoc`) for Heroes of Code maintainers to manage and clone repositories from the Heroes of Code GitHub organization.

## Commands

```bash
# Build
cargo build              # debug build
cargo build --release    # release build

# Run
cargo run                # run the CLI

# Test
cargo test

# Lint & Format
cargo clippy --all-targets --all-features
cargo fmt --all -- --check

# Via mise task runner
mise build
mise test
mise lint
mise fmt
mise check
mise release
mise doc
```

## CLI Usage

```bash
hoc repos   # list all repositories
hoc clone   # clone repositories (interactive)
hoc all     # clone all repositories
hoc pr      # show open pull requests
```

## Architecture

- `src/main.rs` — entry point, sets up tokio async runtime
- `src/lib.rs` — module declarations
- `src/cli.rs` — command router using `clap::Subcommand`
- `src/repos.rs` — fetch and list repositories via GitHub API
- `src/clone.rs` — interactive/batch clone logic, resolves Desktop directory
- `src/pull_requests.rs` — fetch and list open PRs across org
- `src/network.rs` — `reqwest::Client` wrapper
- `src/cli_output.rs` — terminal output helpers (success, error, loading)
- `src/utils.rs` — generic table rendering with `comfy-table`

## Testing

Integration tests live in `tests/` and use `httpmock` to mock HTTP servers. Unit tests (deserialization) live in module files. Test naming convention: `test_<feature>_<scenario>`.

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
- Renovate auto-merges dependency updates on green CI
