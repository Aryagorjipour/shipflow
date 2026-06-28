# shipflow

[![CI](https://github.com/Aryagorjipour/shipflow/actions/workflows/ci.yml/badge.svg)](https://github.com/Aryagorjipour/shipflow/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/shipflow.svg)](https://crates.io/crates/shipflow)
[![docs.rs](https://docs.rs/shipflow/badge.svg)](https://docs.rs/shipflow)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.88+-orange.svg)](rust-toolchain.toml)

**Track what you intend to ship. Celebrate what you actually shipped.**

`shipflow` is a minimalist, local-first, git-aware CLI for developers who want lightweight intention tracking and beautiful personal reflection — not another bloated task manager.

```
   _____ _     _       ______ _
  / ____| |__ (_)___  |  ____| | _____      __
 | (___ | '_ \| / __| | |__  | |/ _ \ \ /\ / /
  \___ \| | | | \__ \ |  __| | | (_) \ V  V /
  ____) | | | | |___/ | |    | |\___/ \_/\_/
 |_____/|_| |_|_|     |_|    |_|\_|

  $ shipflow add "Ship auth refactor" --tags rust,auth
  Added 01JABC12 — Ship auth refactor

  $ shipflow done auth
  Done 01JABC12 — Ship auth refactor
    linked: a1b2c3d Fix OAuth callback redirect

  $ shipflow report week --format md
  # What I shipped — this week
  ...
```

## Why shipflow?

- **Shipping + reflection**, not full project management
- **Git-native** where it helps (branch context, commit linking) — works fine without git
- **Local-first & private** — zero network calls by default
- **Delightful terminal UX** — fast, colorful, respectful of your shell
- **Simple by default, powerful when needed**

## Installation

### Linux / macOS (recommended)

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/Aryagorjipour/shipflow/releases/latest/download/shipflow-installer.sh | sh
```

### Windows (PowerShell)

```powershell
irm https://github.com/Aryagorjipour/shipflow/releases/latest/download/shipflow-installer.ps1 | iex
```

Restart your terminal after install so PATH updates take effect.

**Manual install:** download `shipflow-x86_64-pc-windows-msvc.zip` from [GitHub Releases](https://github.com/Aryagorjipour/shipflow/releases), extract `shipflow.exe`, and add it to your PATH.

**TUI tip:** use [Windows Terminal](https://github.com/microsoft/terminal) for `shipflow board`.

### Cargo

```bash
cargo install shipflow
```

### cargo-binstall

```bash
cargo binstall shipflow
```

### From source

```bash
git clone https://github.com/Aryagorjipour/shipflow
cd shipflow
cargo install --path .
```

Minimal build (no TUI):

```bash
cargo install --path . --no-default-features
```

## Quick start

```bash
# Inside a git repo — tasks stored in .shipflow/tasks.json
shipflow add "Fix parser edge case" --tags rust,cli
shipflow add "Write weekly reflection" --note "Keep it short"

shipflow list --status open
shipflow done parser          # interactive commit linking when TTY
shipflow report week          # beautiful summary
shipflow report week --format md   # paste into standup notes

shipflow status               # counts + git context
shipflow board                # optional kanban TUI
```

### Shell completions

```bash
shipflow completions fish > ~/.config/fish/completions/shipflow.fish
shipflow completions bash > ~/.local/share/bash-completion/completions/shipflow
shipflow completions zsh > ~/.zfunc/_shipflow
```

```powershell
shipflow completions powershell >> $PROFILE
```

## Storage modes

| Mode | Path | When |
|------|------|------|
| **Repo** (default in git) | `.shipflow/tasks.json` | Inside a git repository |
| **Global** | `~/.config/shipflow/tasks.json` | Outside git, or with `--global` |

**Privacy tip:** add `.shipflow/` to `.gitignore` to keep intentions local:

```gitignore
.shipflow/
```

Remove from `.gitignore` if you want team-visible shipped work in the repo.

## Commands

| Command | Description |
|---------|-------------|
| `add` | Add a task (`--tags`, `--note`, `--global`) |
| `list` | List tasks (`--status open\|done\|all`, `--tags`) |
| `done` | Mark done (`--commit`, `--no-link`, `--global`) |
| `report` | What I shipped (`today\|week\|month\|all`, `--format text\|md`) |
| `status` | Overview + git context |
| `board` | Kanban TUI (Open / Done) |
| `completions` | Generate shell completions |

## Environment variables

| Variable | Effect |
|----------|--------|
| `NO_COLOR` | Disable colored output |
| `CLICOLOR=0` | Disable colored output |
| `RUST_LOG` | Enable tracing (e.g. `shipflow=debug`) |

## Development

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## MSRV

Minimum Supported Rust Version: **1.88** (see `rust-version` in `Cargo.toml`).

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.