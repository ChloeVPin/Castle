# Changelog

All notable changes to Castle are documented here.
Format based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

Expansion of the original 2-day, ~200-line project manager into a small C++
toolchain helper. The original `new`/`build`/`run` commands remain backward
compatible; everything below is additive.

### Changed — presentation
- README first screen now states what Castle is, shows a `Castle:` transcript,
  and uses a live CI badge. Origin credit lives under Acknowledgments.
- README header is the keep, the name, a one-line description, then badges.
- Errors print `Castle: error: …`, matching the existing `warning:` label.
- `Cargo.toml` now has description, license, repository, keywords, and
  `rust-version = "1.87"`.
- Dropped the decorative castle emoji from clap / rustdoc about text.

### Added — Tier 1 (foundations)
- **`clap` derive CLI** with real `--help`, `--version` (`castle -V`), and
  per-subcommand help.
- **`castle init [name]`** — scaffold in the current directory (`cargo init` style).
- **`castle clean [--force]`** — remove `build/` and `compile_commands.json`,
  with a confirmation prompt and a non-interactive guard.
- **`castle check`** — fast `-fsyntax-only` type/syntax pass over every source,
  honoring the project's real flags.
- **Build profiles** — `--release` / `--debug` mapped to optimization + debug info.
- **`--cxx-standard=23|26`** (and `20`) with a clang-version guard per standard.
- **`castle.toml` manifest** (`[package]`, `[profile]`, optional `backend`),
  parsed with `serde` + `toml`, every field defaulted, backward compatible.
- **`castle version`** — prints Castle version, detected clang line, C++ standard,
  and available tools (cmake/ccache/clang-format/clang-tidy).

### Added — Tier 2 (essence)
- **Hybrid build backend**: `clang++` direct (default, minimal, needs only clang)
  and `cmake` (opt-in via manifest or `--backend cmake`). Both share one flag
  derivation so they never drift.
- **`compile_commands.json` for clangd**, written to the project root by both
  backends (the clang backend hand-rolls it; the cmake backend mirrors CMake's).
- **Typed `CastleError`** enum (`thiserror`) with friendly `Castle:`-prefixed
  messages and Cargo-style `hint:` lines. No panics on user input.
- **Colored output** that respects `NO_COLOR`, `CI`, `TERM=dumb`, and TTY
  detection.
- **Sanitizer & warning presets** — `--asan`, `--tsan`, `--ubsan`, `--msan`,
  `--pedantic`, plus `[profile] warnings_as_errors` and `sanitizer`.
- **`castle test`** — builds and runs every `*.cpp` under `tests/`; the scaffolded
  `tests/main.cpp` uses plain `assert` (no framework dependency).
- **`castle fmt`** — wraps `clang-format -i` and drops a default `.clang-format`
  (LLVM-based, C++23-aware) on first run.
- **`castle tidy`** — wraps `clang-tidy -p .` using `compile_commands.json` and
  drops a curated default `.clang-tidy`.
- **`castle run --watch`** — rebuilds (and re-runs) on `src/` changes via `notify`,
  with a manual debounce.

### Added — project hygiene
- **Module split**: `cli`, `commands/*`, `manifest`, `toolchain`, `backend`,
  `project`, `templates`, `error`, `output`, `shell`.
- **Templates** moved under `src/templates/`, still loaded via `include_str!`.
- **Unit tests** for pure logic (clang-version parsing, manifest parsing, flag
  mapping, backend resolution, source discovery, template rendering).
- **End-to-end integration test** (`tests/round_trip.rs`) that scaffolds a real
  project in a tempdir and runs `new → build → run → check → test → clean`
  against the built binary.
- **CI workflow** (`.github/workflows/ci.yml`) — macOS + Ubuntu, clang 18+,
  running fmt + clippy + tests + a sample round-trip.
- **Docs**: rewritten `README.md`, `ARCHITECTURE.md`, `CONTRIBUTING.md`,
  `examples/`.

### Changed
- Compile and link now use **`clang++`** (the C++ driver) instead of `clang`,
  so the C++ standard library (libc++/libstdc++) is linked automatically on
  macOS and Linux.
- Project-root discovery keys off `castle.toml` **or** `CMakeLists.txt`
  (backward compatible with 0.1.0 projects).
- `cargo install --path .` now pulls a small, documented dependency set
  (see README "Dependencies & why").

### Added — cross-platform
- **Windows support** — `.exe` suffix handling in `find_built_binary`, `on_path`
  probes `.exe` on Windows, `\` rejection in project-name validation, cmake
  backend forces `-DCMAKE_CXX_COMPILER=clang++` so it doesn't default to MSVC.
- **CI matrix**: macOS, Ubuntu, **Windows** — full round-trip on all three OS.
- **Cross-compile CI job** (`cross-check`) — `cargo check --all-targets` against
  `x86_64-pc-windows-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`,
  `aarch64-unknown-linux-gnu` on every push.

### Notes
- clang < 18 remains unsupported, matching the original scope.
- C++23 modules and full dependency resolution are documented as roadmap items,
  not implemented.

## [0.1.0] — the original

The original ~200-line learning milestone.

### Added
- `castle new <project_name>` — scaffolds `src/main.cpp` (C++23 `std::println`),
  `.clangd`, and `CMakeLists.txt`.
- `castle build` — checks for clang ≥ 18, walks up to find `CMakeLists.txt`,
  runs `cmake -S . -B build` then `cmake --build build`.
- `castle run` — executes `build/bin`.
- `include_str!`-loaded `.txt` templates; friendly `Castle:`-prefixed errors;
  `ExitCode` returns.
