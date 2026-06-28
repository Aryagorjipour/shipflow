# ADR-0001: Core Architecture Decisions

**Status:** Accepted  
**Date:** 2026-06-28

## Context

shipflow v0.1 needs local-first task storage, optional git integration, and a fast CLI UX without unnecessary dependencies.

## Decisions

### ADR-001: Serialization — Pretty JSON

Use `tasks.json` with `serde_json` pretty printing.

**Rationale:** Universal tooling (`jq`, editors), predictable git diffs, zero learning curve. RON deferred to v0.2 import/export.

### ADR-002: Git — subprocess, not libgit2

Use `std::process::Command("git", ...)`.

**Rationale:** MVP only needs repo root, branch, and recent log. Avoids libgit2 binary bloat and compile time.

### ADR-003: Edition / MSRV — Rust 2024, MSRV 1.88

Edition 2024 with `rust-version = "1.88"` (comfy-table 7.2 requires recent stable).

### ADR-004: Task IDs — ULID

Sortable, collision-resistant, merge-friendly in per-repo JSON.

### ADR-005: Storage — per-repo default, global fallback

- Inside git repo: `<repo>/.shipflow/tasks.json`
- Outside git: `~/.config/shipflow/tasks.json` (XDG via `directories`)
- `--global` flag overrides repo mode

### ADR-006: Privacy — gitignore `.shipflow/` by default

README documents private (gitignored) vs team-visible (committed) workflows.

### ADR-007: License — MIT OR Apache-2.0

Dual license per Rust ecosystem convention.

### ADR-008: Interactive `done` — TTY stdin menu

Numbered commit list when TTY; `--commit` and `--no-link` for scripting. No extra prompt crate.

### ADR-009: TUI — `board` behind `tui` feature (default-on)

`--no-default-features` builds without ratatui for minimal installs.

### ADR-010: Logging — tracing, silent by default

`RUST_LOG` enables structured logs; user-facing errors use `thiserror` + `owo-colors`.

### ADR-011: Colors — owo-colors + TTY detection

Respects `NO_COLOR`, `CLICOLOR=0`, and disables color when stdout is not a TTY.

## Consequences

- Fast cold starts and small release binaries
- Works without git installed
- Schema `version` field enables future migrations