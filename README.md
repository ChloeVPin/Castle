<p align="center">
  <img src="hero.png" alt="Castle" width="400" />
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.87%2B-orange?logo=rust" alt="Rust 1.87+" />
  <img src="https://img.shields.io/badge/clang-18%2B-blue?logo=llvm" alt="Clang 18+" />
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey" alt="Platform" />
  <img src="https://img.shields.io/badge/standard-C%2B%2B23-00599C?logo=c%2B%2B" alt="C++23" />
</p>

---

## Castle

Castle is a small C++ project manager written in Rust. Its command structure is similar to Cargo.

Castle can create, build, test, and run C++ projects. By default, it uses Clang and C++23. It also supports build profiles, sanitizers, clangd, watch mode, formatting, static analysis, and CMake.

> **Project origin**  
> This project started from [sunless2day/Castle](https://github.com/sunless2day/Castle). The original project was a small Rust learning project with the `new`, `build`, and `run` commands. These commands continue to work. This repository adds the other functions that are described below. Credit for the original foundation belongs to the original author.

---

## Navigation

| Section | Content |
|---|---|
| [Features](#features) | Commands and flags |
| [Quick Start](#quick-start) | Basic installation and use |
| [Install](#install) | Platform requirements |
| [Manifest](#manifest) | `castle.toml` reference |
| [Backends](#backends) | Clang and CMake backends |
| [Comparison](#comparison) | Comparison with other tools |
| [Roadmap](#roadmap) | Planned work |
| [Development](#development) | Build, test, and contribution commands |
| [Acknowledgments](#acknowledgments) | Project credits |

---

## Features

| Command | Function |
|---|---|
| `castle new <name>` | Create a project in a new directory. |
| `castle init [name]` | Create a project in the current directory. |
| `castle build` | Compile and link the project. |
| `castle run [args]` | Build and run the binary. Supports `--watch`. |
| `castle check` | Run a fast `-fsyntax-only` check without linking. |
| `castle test` | Build and run files under `tests/`. |
| `castle clean [--force]` | Remove `build/` and `compile_commands.json`. |
| `castle fmt` | Run `clang-format` on `src/` and `tests/`. |
| `castle tidy` | Run `clang-tidy` with `compile_commands.json`. |
| `castle version` | Print the Castle version and detected toolchain information. |
| `castle --help` | List commands and descriptions. |

Build flags:

- `--release`
- `--cxx-standard=20|23|26`
- `--backend clang|cmake`
- `--asan`
- `--tsan`
- `--ubsan`
- `--msan`
- `--pedantic`

Debug mode is the default when you do not use `--release`.

---

## Quick Start

```bash
cargo install --path .
export PATH="$HOME/.cargo/bin:$PATH"

castle new hello
cd hello
castle build
castle run
```

Output:

```text
Hello, world! Your castle stands.
```

After the first `castle build`, Castle writes `compile_commands.json` in the project root. clangd reads this file automatically. No extra configuration is necessary in VS Code, Neovim, or Helix.

---

## Install

### Requirements

| Requirement | Notes |
|---|---|
| Rust 1.87 or later | Uses Rust edition 2024 and let-chains. Run `rustup install stable`. |
| Clang 18 or later | Castle uses Clang for version detection. |
| `clang++` on `PATH` | Castle uses the C++ driver to compile and link the standard library. |

### Platform Commands

| Platform | Command |
|---|---|
| macOS | `xcode-select --install` |
| Ubuntu or Debian | `sudo apt install clang-18 clang++-18` |
| Windows | `choco install llvm` or download LLVM from [llvm.org](https://llvm.org) |

### Optional Tools

| Tool | Used by |
|---|---|
| CMake | `castle build --backend cmake` |
| clang-format | `castle fmt` |
| clang-tidy | `castle tidy` |
| ccache | Castle can identify it and show a build hint. |

Install Castle:

```bash
cargo install --path .
```

---

## Manifest

Castle has defaults for all fields. A project without `castle.toml` can still build. The `castle new` and `castle init` commands create a commented manifest.

```toml
backend = "clang"          # "clang" or "cmake"

[package]
name = "hello"             # Output binary name
version = "0.1.0"          # Informational version
cxx_standard = 23          # 20, 23, or 26

[profile]
opt_level = "0"            # "0", "1", "2", "3", or "s"
debug_info = true          # Add -g in non-release builds
warnings_as_errors = false
pedantic = true            # Add -Wpedantic
sanitizer = "address"      # "address", "thread", "undefined", or "memory"
```

CLI flags override the manifest. For example, `castle build --release --cxx-standard=26` overrides the profile and `cxx_standard` values.

The `--pedantic` flag turns on pedantic warnings. When you omit this flag, Castle uses the manifest value.

---

## Backends

Castle has two build backends. Both backends use the same flag-generation logic.

| Backend | Operation | Recommended use |
|---|---|---|
| `clang` | Calls `clang++` directly. It compiles each translation unit to `build/` and then links the binary. | Small projects and quick development |
| `cmake` | Creates a `CMakeLists.txt` file and runs `cmake --build`. | Multi-file projects and IDE integration |

Set `backend = "cmake"` in `castle.toml`, or use `--backend cmake` for one command. Both backends write `compile_commands.json` to the project root.

---

## Comparison

| Capability | Castle | CMake | Meson | Cargo |
|---|---|---|---|---|
| Main language | C++ | C, C++, and others | Multiple | Rust |
| Manifest | `castle.toml` | `CMakeLists.txt` | `meson.build` | `Cargo.toml` |
| Single build command | Yes | No | Yes | Yes |
| Project creation | Yes | No | Yes | Yes |
| clangd configuration | Writes `compile_commands.json` | Requires a flag | Yes | Not applicable |
| Scope | Small and opinionated | Full build system | Full build system | Full Rust project manager |

Castle does not replace CMake or Meson. It provides a smaller interface and can use CMake as a backend.

---

## Roadmap

See `ARCHITECTURE.md` and `CHANGELOG.md` for more information.

- **Dependencies**: Add `castle add <git-url>` with CMake `FetchContent`.
- **Libraries**: Add `castle new --lib`, header-only and compiled libraries, and a `[targets]` table.
- **Templates**: Add `castle new --template=<name>` with a user `templates/` directory.
- **Workspaces**: Add multi-project `[workspace]` support.
- **C++23 modules**: Add dependency scanning for `import` statements and BMI build order.
- **ccache**: Add automatic ccache use for the Clang backend.
- **Shell completions and manual page**: Use `clap_complete` and `clap_mangen`.

C++23 module support is a long-term goal and can require substantial compiler-specific work.

---

## Dependencies

| Crate | Function |
|---|---|
| `clap` | Parse commands, flags, and help text. |
| `serde` and `toml` | Parse `castle.toml` and apply defaults. |
| `serde_json` | Write `compile_commands.json`. |
| `thiserror` | Define typed Castle errors. |
| `notify` | Rebuild after file changes in watch mode. |
| `tempfile` | Create temporary projects for integration tests. |

Castle does not use an asynchronous runtime or a large build-graph library.

---

## Development

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps --open
```

See `CONTRIBUTING.md` for the module structure and command-extension instructions. See `ARCHITECTURE.md` for the control flow.

---

## CI

| Job | Targets and checks |
|---|---|
| `check` | macOS, Ubuntu, and Windows: formatting, Clippy, tests, and a live round trip |
| `cross-check` | `x86_64-pc-windows-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, and `aarch64-unknown-linux-gnu`: `cargo check --all-targets` |

---

## Acknowledgments

Castle is based on [sunless2day/Castle](https://github.com/sunless2day/Castle). The original project was a small C++ project manager written in Rust as a learning project. Its clear structure made this extension possible.

---

## License

MIT. See [LICENSE](LICENSE). This project includes work based on [sunless2day/Castle](https://github.com/sunless2day/Castle), which was published as a public learning resource.
