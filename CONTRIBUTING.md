# Contributing to Castle

Castle is intentionally small. Contributions should preserve that constraint unless a larger change has a clear project-level justification.

## Setup

```bash
git clone <repo>
cd Castle
cargo install --path .
```

Prerequisites:

- stable Rust **1.88 or later** (edition 2024; Castle uses let-chains)
- Clang **18 or later**
- `clang++` on `PATH`

See the README for platform installation notes.

## Development loop

```bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

`cargo test` includes `tests/round_trip.rs`, which scaffolds, compiles, and runs a real C++ project through the built `castle` binary. A working `clang++` is required for the full suite.

## Repository map

See `ARCHITECTURE.md` for the full module map. The main extension points are:

- `src/cli.rs` — clap command and flag definitions.
- `src/commands/<name>.rs` — one handler per subcommand.
- `src/commands/common.rs` — shared configuration and validation helpers.
- `src/backend.rs` — direct Clang and CMake backends plus shared flag derivation.
- `src/output.rs` — terminal output policy and `NO_COLOR`/TTY behavior.
- `src/templates/` — text templates embedded with `include_str!`.

## Adding a command

1. Define the CLI surface in `src/cli.rs`.
2. Create `src/commands/<name>.rs` with a `pub fn run(...) -> error::Result<()>` handler.
3. Register the module and re-export in `src/commands/mod.rs`.
4. Dispatch it from `src/main.rs`.
5. Add unit tests for pure logic and extend `tests/round_trip.rs` when the command has an end-to-end workflow.
6. Update README documentation and the changelog when the user-facing surface changes.

Use `Printer` for Castle-owned terminal output and `shell::run_step` for subprocess steps. Compiler and tool diagnostics should continue to inherit their normal streams rather than being reformatted by Castle.

## Adding a template

1. Add a `.txt` file under `src/templates/` using `{{PLACEHOLDER}}` markers when substitution is required.
2. Embed it from `src/templates.rs` with `include_str!`.
3. Render substitutions through `templates::render`.
4. Add or update template smoke tests.

## Style and compatibility

- Keep user-facing output concise and use the established `Castle:` prefix through `Printer`.
- Preserve `NO_COLOR`, `CI`, `TERM=dumb`, and TTY behavior.
- Avoid `unwrap` and `expect` on user-facing paths; return typed errors instead.
- Add rustdoc to new public items.
- Prefer small, reviewable modules and focused diffs.
- Do not add a dependency solely for terminal appearance or decoration.
- Keep direct Clang as the default backend and CMake as the explicit escape hatch unless a change intentionally revisits that product decision.

## Commit messages

Use short conventional prefixes such as `feat:`, `fix:`, `docs:`, `test:`, `refactor:`, and `chore:`. Add a scope only when it improves clarity.

## Scope

Open an issue before starting a substantial new backend, dependency-resolution system, workspace model, or C++ modules implementation. Castle should ship a small complete capability rather than a broad incomplete one.
