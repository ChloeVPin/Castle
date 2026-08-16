<p align="center">
  <img src="hero.png" alt="Castle — a minimal C++ project manager" width="400" />
</p>

# Castle

Cargo-inspired project management for small C++ executables, with direct Clang builds by default and CMake when you need it.

Castle keeps the common loop small: scaffold a project, build it, run it, test it, and keep clangd tooling in sync without introducing a separate build-graph dependency.

```text
$ castle new hello
Castle: created project `hello`
  hint: cd into it and run `castle build`

$ cd hello && castle run
Castle: clang 18, c++23, debug, clang backend
Castle: compile src/main.cpp
Castle: link build/hello
Castle: build finished
Castle: run build/hello
Hello, world! Your castle stands.
```

## Why Castle

- **Direct Clang by default.** Small projects compile and link through `clang++` without a generated build system.
- **One project workflow.** `new`, `build`, `run`, `check`, `test`, `fmt`, `tidy`, and `clean` use one CLI and one manifest.
- **CMake when needed.** Switch the backend per project or per command without changing Castle's flag derivation.
- **clangd-ready.** Both backends maintain `compile_commands.json` at the project root.

Castle is deliberately narrower than CMake or Meson. It is intended for small executable projects that benefit from a Cargo-like workflow, not as a replacement for a general-purpose build system.

## Install

### Requirements

- Rust **1.88 or later**
- Clang **18 or later**
- `clang++` on `PATH`

Platform examples:

```bash
# macOS
xcode-select --install

# Ubuntu / Debian with LLVM 18 packages configured
sudo apt install clang-18

# Windows
choco install llvm
```

Install Castle from a checkout:

```bash
cargo install --path .
```

Optional tools are CMake for the CMake backend, `clang-format` for `castle fmt`, and `clang-tidy` for `castle tidy`.

## Quick start

```bash
castle new hello
cd hello
castle build
castle run
```

After the first build, Castle writes `compile_commands.json` at the project root for clangd-compatible editors.

## Commands

| Command | Purpose |
|---|---|
| `castle new <name>` | Create a project in a new directory. |
| `castle init [name]` | Create a project in the current directory. |
| `castle build` | Compile and link the project. |
| `castle run [args]` | Build and run the binary; supports `--watch`. |
| `castle check` | Run `-fsyntax-only` over project sources. |
| `castle test` | Build and run files under `tests/`. |
| `castle clean [--force]` | Remove build artifacts and `compile_commands.json`. |
| `castle fmt` | Run `clang-format` on `src/` and `tests/`. |
| `castle tidy` | Run `clang-tidy` using `compile_commands.json`. |
| `castle version` | Print Castle and detected toolchain information. |

Common build flags include `--release`, `--cxx-standard=20|23|26`, `--backend clang|cmake`, sanitizer flags, and `--pedantic` for `-Wpedantic`.

## Configuration

`castle.toml` is optional; Castle has defaults for all fields. `castle new` and `castle init` create a commented manifest.

```toml
backend = "clang"

[package]
name = "hello"
version = "0.1.0"
cxx_standard = 23

[profile]
opt_level = "0"
debug_info = true
warnings_as_errors = false
pedantic = true
```

CLI flags override manifest values. See [docs/configuration.md](docs/configuration.md) for the full manifest and sanitizer reference.

## Backends

| Backend | What it does | Best fit |
|---|---|---|
| `clang` | Calls `clang++` directly, compiles translation units into `build/`, then links. | Small projects and fast iteration. |
| `cmake` | Generates `CMakeLists.txt`, configures CMake, and invokes `cmake --build`. | Projects that need CMake or broader IDE/build-system integration. |

Set `backend = "cmake"` in `castle.toml`, or use `--backend cmake` for one command. Both backends use the same Castle flag derivation and maintain `compile_commands.json`.

## Where Castle fits

| | Raw Clang | Castle | CMake |
|---|---|---|---|
| Project scaffolding | Manual | Built in | Manual / external |
| Direct compiler workflow | Yes | Default | No |
| Repeatable project configuration | Scripts / flags | `castle.toml` | `CMakeLists.txt` |
| Build command | Manual invocation | `castle build` | `cmake --build <dir>` after configuration |
| `compile_commands.json` | Manual | Maintained by Castle | Available through CMake configuration |
| Scope | Compiler driver | Small project manager | General-purpose build system |

## Documentation

| Document | Purpose |
|---|---|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Module map, control flow, and implementation boundaries. |
| [docs/configuration.md](docs/configuration.md) | Manifest, profiles, sanitizers, and backend configuration. |
| [examples/](examples/) | Default and CMake-backed sample projects. |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Local development and contribution workflow. |
| [CHANGELOG.md](CHANGELOG.md) | Notable project changes. |

## Scope

Castle focuses on small C++ executable projects and a predictable local toolchain workflow. Dependency resolution, workspaces, library/package publishing, and full C++ module dependency scanning are outside the current feature set. Concrete future work is tracked as repository issues rather than promised here.

## Development

```bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

The integration suite scaffolds and exercises real C++ projects, so a working `clang++` is required for the full test run.

## Acknowledgments

Castle is based on [sunless2day/Castle](https://github.com/sunless2day/Castle). The original project established the small Rust/C++ project-manager foundation that this repository extends.

## License

MIT. See [LICENSE](LICENSE).
