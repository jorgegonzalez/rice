# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Rice is a neofetch-style system information CLI written in Rust (2021 edition, MSRV 1.88). Published to crates.io as `rice` and built as both a library (`src/lib.rs`) and a binary (`src/main.rs`).

There is a more detailed `AGENTS.md` in the repo root that mirrors much of the architecture below — when editing architecture, keep both files in sync.

## Common commands

```bash
cargo build                                          # debug build
cargo build --release                                # release build
cargo run -- --help                                  # run with args
cargo test                                           # run all tests
cargo test format_uptime                             # run tests matching a name
cargo test --lib format_bytes -- --nocapture         # run a single lib test, show stdout
cargo fmt --all -- --check                           # CI formatting check
cargo clippy --all-targets --all-features -- -D warnings   # CI lint (warnings → errors)
cargo audit                                          # CI security audit (cargo install cargo-audit first)
docker compose up --build                            # build & run via docker
```

CI (`.github/workflows/ci.yml`) runs fmt-check, clippy with `-D warnings`, build, test, and release-build on ubuntu/windows/macos. Anything that doesn't pass `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` will fail CI.

## Architecture

Three layers, wired together in `src/main.rs::run_modern_mode`:

1. **`config/`** — TOML config (`Config`, `DisplayConfig`, `InfoConfig`, `AsciiArtConfig`). `loader.rs` resolves the path (`~/.config/rice/config.toml` on Unix, `%APPDATA%/rice/config.toml` on Windows, or `--config <path>`); `defaults.rs` owns the embedded default TOML used when generating a new config via `rice config`.
2. **`info/`** — `InfoCollector` takes an ordered `fields: Vec<String>` plus a `custom_commands: HashMap<String, String>` and produces a `HashMap<String, String>`. `collect_field` first checks `custom_commands`, then falls through to built-in field handlers split across `system.rs` (os/hostname/userhost/kernel/uptime), `hardware.rs` (cpu/memory/disk), `software.rs` (shell/terminal/packages/resolution/de/wm), and a special `colors` block built inline. Unknown or failing fields are silently skipped — that is intentional, do not "fix" it by surfacing errors.
3. **`display/`** — `Display::render` lays out ASCII art on the left and the collected fields on the right, **respecting the order in `config.info.fields`** (not a hardcoded list). Per-field colors come from `display.field_colors`. `layout.rs` and `themes.rs` exist but are currently unused scaffolding for planned alternative layouts.

`main.rs` also keeps a parallel **legacy command path** (`system`, `cpu`, `memory`, `disk`, `network` subcommands → `run_legacy_mode`). It hits `sysinfo` directly and does its own formatting; do not refactor it into the modular path without a plan — it exists for backwards CLI compatibility.

`lib.rs` exposes the modules publicly plus two formatting helpers (`format_bytes`, `format_uptime`) which are where the unit tests live.

## Conventions worth knowing

- **Honest config.** Do not add config keys that don't actually do something. The default TOML in `config/defaults.rs` and `config.example.toml` are user-facing — they should match what the code reads.
- **Field order is data, not code.** Display iteration must come from `config.info.fields`. Custom commands appear inline at the position they're configured.
- **Custom commands are user shell snippets** executed as-is (see `info/custom.rs`). Output is truncated to 100 chars and failures are swallowed — these are deliberate UX choices.
- **Cross-platform.** Use `cfg!(target_os = "...")` / `cfg!(windows)` for platform branches. macOS-specific code already exists in `info/system.rs`.
- **Image support is feature-gated** behind the `images` feature (default-on), pulling in `image` and `base64`. Anything image-related must compile with `--no-default-features` too.

## Release

`release-plz.toml` configures release-plz; `.github/workflows/release.yml` handles the publish flow. Version lives in `Cargo.toml`. `CHANGELOG.md` is updated by release-plz — don't hand-edit unless the workflow is broken.
