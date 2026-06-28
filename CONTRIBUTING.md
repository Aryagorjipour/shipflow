# Contributing to shipflow

Thanks for your interest in contributing!

## Getting started

1. Fork and clone the repository
2. Install Rust 1.88+ (see `rust-toolchain.toml`)
3. Run the test suite:

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Pull requests

- Use [Conventional Commits](https://www.conventionalcommits.org/) for commit messages (`feat:`, `fix:`, `docs:`, `chore:`, etc.)
- Keep PRs focused — one logical change per PR
- Add or update tests for behavior changes
- Update README/docs when user-facing behavior changes

## Architecture changes

Non-trivial design decisions should be documented as ADRs in `docs/adr/`.

## Code style

- `cargo fmt` before committing
- No `unwrap()` / `expect()` in production code (enforced by clippy)
- Respect terminal conventions (`NO_COLOR`, pipe detection)

## Reporting issues

Include your OS, `shipflow --version`, and steps to reproduce. Redact any private task titles if needed.