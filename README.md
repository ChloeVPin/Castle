<div align="center">
  <img src="logo.png" alt="Castle logo" width="144" />

  <h1>Castle</h1>

  <p>Cargo for small C++ projects.</p>

  <p>
    <a href="https://github.com/ChloeVPin/Castle/actions/workflows/ci.yml"><img src="https://github.com/ChloeVPin/Castle/actions/workflows/ci.yml/badge.svg" alt="CI" /></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-lightgrey" alt="MIT license" /></a>
    <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey" alt="macOS | Linux | Windows" />
  </p>
</div>

Castle is a Cargo-inspired C++ project manager. It scaffolds, builds, tests, and runs small C++ projects with Clang and C++23 by default.

```text
$ castle new hello && cd hello && castle build
Castle: using clang 18 (-std=c++23, via clang++), backend = clang
Castle: clang compile src/main.cpp
Castle: clang link hello
Castle: build finished
```

`castle new` writes a working tree. `castle build` compiles it and drops `compile_commands.json` at the project root so clangd works in VS Code, Neovim, or Helix with no extra setup.

Castle is not a CMake replacement. It is a smaller interface — and it can call CMake when you want that backend.

## Quick start

```bash
cargo install --path .
export PATH="$HOME/.cargo/bin:$PATH"

castle new hello
cd hello
castle build
castle run
```

```text
Hello, world! Your castle stands.
```

## Commands

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

## Backends

Castle has two build backends. Both backends use the same flag-generation logic.

| Backend | Operation | Recommended use |
|---|---|---|
| `clang` | Calls `clang++` directly. It compiles each translation unit to `build/` and then links the binary. | Small projects and quick development |
| `cmake` | Creates a `CMakeLists.txt` file and runs `cmake --build`. | Multi-file projects and IDE integration |

Set `backend = "cmake"` in `castle.toml`, or use `--backend cmake` for one command. Both backends write `compile_commands.json` to the project root.

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

## More

| | |
|---|---|
| [Install](#install) | Toolchain requirements |
| [Roadmap](#roadmap) | Planned work |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Control flow and module map |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to extend Castle |
| [CHANGELOG.md](CHANGELOG.md) | What changed |
| [examples/](examples/) | `hello` and a multi-file CMake project |

CI runs format, Clippy, tests, and a live `new → build → run` round-trip on macOS, Ubuntu, and Windows, plus `cargo check` for additional targets. See `.github/workflows/ci.yml`.

## Install

### Requirements

| Requirement | Notes |
|---|---|
| Rust 1.87 or later | Uses Rust edition 2024 and let-chains. Run `rustup install stable`. |
| Clang 18 or later | Castle uses Clang for version detection. |
| `clang++` on `PATH` | Castle uses the C++ driver to compile and link the standard library. |

### Platform commands

| Platform | Command |
|---|---|
| macOS | `xcode-select --install` |
| Ubuntu or Debian | `sudo apt install clang-18 clang++-18` |
| Windows | `choco install llvm` or download LLVM from [llvm.org](https://llvm.org) |

### Optional tools

| Tool | Used by |
|---|---|
| CMake | `castle build --backend cmake` |
| clang-format | `castle fmt` |
| clang-tidy | `castle tidy` |
| ccache | Castle can identify it and show a build hint. |

```bash
cargo install --path .
```

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

## Acknowledgments

Castle is based on [sunless2day/Castle](https://github.com/sunless2day/Castle). The original project was a small C++ project manager written in Rust as a learning project, with `new`, `build`, and `run`. Those commands still work. This repository adds the rest of the surface described above. Credit for the original foundation belongs to the original author.

## License

MIT. See [LICENSE](LICENSE). This project includes work based on [sunless2day/Castle](https://github.com/sunless2day/Castle), which was published as a public learning resource.
