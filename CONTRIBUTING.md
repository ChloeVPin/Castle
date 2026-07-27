# Contributing to Castle

Thanks for hacking on the keep! Castle is intentionally small, so most
contributions should stay small too.

## Setup

```bash
git clone <repo>
cd Castle
cargo install --path .          # optional: put `castle` on your PATH
```

Prerequisites: a stable Rust toolchain **≥ 1.87** (edition 2024; the code uses
let-chains, stabilized in 1.87), clang ≥ 18 *and*
`clang++` on `PATH`. See the README for platform install notes.

## The loop

```bash
cargo fmt                              # format
cargo clippy --all-targets -- -D warnings   # lint (warnings are errors here)
cargo test                             # unit tests + end-to-end round-trip
cargo doc --no-deps --open             # docs
```

`cargo test` includes `tests/round_trip.rs`, which actually compiles and runs a
scaffolded C++ project through the `castle` binary — so you need a working
`clang++` for the full suite to pass.

## Where things live

See `ARCHITECTURE.md` for the full module map. In short:

- `src/cli.rs` — the clap command/flag definitions.
- `src/commands/<name>.rs` — one file per subcommand.
- `src/commands/common.rs` — shared helpers (`resolve_build_config`,
  `validate_project_name`, `min_clang_for`).
- `src/backend.rs` — the clang/cmake backends and `Flags` derivation.
- `src/templates/` — the `.txt` files baked in via `include_str!`.

## How to add a command

1. **Define the CLI** in `src/cli.rs`: add a variant to the `Command` enum with
   its args/flags (clap derive).
2. **Create the handler** at `src/commands/<name>.rs` with a `pub fn run(...)`
   that returns `error::Result<()>`. Use `Printer` for all output and
   `shell::run_step` for subprocesses.
3. **Register it** in `src/commands/mod.rs` (`pub mod <name>;` and a
   `pub use <name>::run as run_<name>;`).
4. **Dispatch it** in `src/main.rs`'s `dispatch()`.
5. **Test it**: add unit tests for pure logic; extend `tests/round_trip.rs` if
   it has an end-to-end story.
6. **Document it**: add a row to the README feature table and a CHANGELOG entry.

## How to add a template

1. Drop a `.txt` file into `src/templates/` (use `{{PLACEHOLDER}}` markers).
2. Add `pub const NAME: &str = include_str!("templates/<file>.txt");` in
   `src/templates.rs`.
3. Use `templates::render(NAME, &[("KEY", value)])` to substitute placeholders.
4. Add a smoke assertion to `templates::tests::all_templates_are_embedded`.

## Style

- Keep the `Castle:` voice in all user-facing messages (use `Printer`).
- No `unwrap`/`expect` on user-facing paths — use `?` with typed errors.
- New public items get rustdoc.
- Keep diffs reviewable; prefer small, well-named modules over megafiles.
- Run `cargo fmt` and `cargo clippy --all-targets -- -D warnings` before
  committing.

## Commit messages

Use conventional prefixes: `feat(cli): …`, `fix(backend): …`, `docs: …`,
`test: …`, `chore: …`. Keep the subject line short.

## Scope

If you're adding something big (a new backend, dependency resolution, C++23
modules), open an issue first. Castle's whole pitch is *minimal but
essential* — we'd rather ship a small, honest slice than a sprawling half-done
feature.
