# Changelog

All notable changes to Castle are documented here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- clap-derived CLI help and version output.
- `castle init [name]` for scaffolding in the current directory.
- `castle clean [--force]` with a confirmation prompt and non-interactive guard.
- `castle check` for `-fsyntax-only` source checks.
- `castle test` for compiling and running `tests/*.cpp` without a test-framework dependency.
- `castle fmt` and `castle tidy` wrappers for clang-format and clang-tidy.
- `castle run --watch` with source-change rebuilds through `notify`.
- `castle version` with detected Clang and optional-tool information.
- `castle.toml` project configuration with package, profile, sanitizer, warning, backend, and C++ standard settings.
- Direct `clang++` and CMake build backends sharing the same flag derivation.
- `compile_commands.json` generation or mirroring for clangd-compatible tooling.
- Sanitizer presets for AddressSanitizer, ThreadSanitizer, UndefinedBehaviorSanitizer, and MemorySanitizer.
- Typed `CastleError` failures and contextual `hint:` output.
- Terminal color that respects `NO_COLOR`, `CI`, `TERM=dumb`, and TTY detection.
- Windows executable handling and LLVM/CMake CI coverage.
- End-to-end integration coverage that exercises `new`, `build`, `run`, `check`, `test`, and `clean` against a real scaffolded project.
- Cross-target `cargo check` coverage for selected Windows, macOS, and Linux targets.

### Changed

- Compile and link operations use `clang++`, ensuring the C++ standard library is linked through the C++ driver.
- Project discovery accepts either `castle.toml` or `CMakeLists.txt` as a project-root marker.
- Source templates live under `src/templates/` and remain embedded with `include_str!`.
- User-facing documentation now separates the core workflow from detailed configuration reference material.
- The declared minimum Rust version is 1.88, matching the edition-2024 let-chain usage in the codebase.
- Generated `.clang-format` configuration avoids a version-specific `Standard` value so the supported clang-format 18 toolchain can consume it.
- Error output uses an explicit `error:` label and command failures are reported once.

## [0.1.0]

### Added

- `castle new <project_name>` for scaffolding a small C++ project.
- `castle build` for locating a project and invoking its build flow.
- `castle run` for executing the built binary.
- Embedded text templates and `Castle:`-prefixed user-facing errors.
