# Castle

A Cargo-inspired project manager for small C++ projects. Castle scaffolds, builds, checks, tests, and runs projects with Clang and C++23 by default.

## Start here

Requirements are Rust, Clang, and the tools used by any optional CMake backend.

```sh
cargo install --path .
castle new hello
cd hello
castle build
castle run
```

A new project can build without a manifest. When one is present, castle.toml controls the backend, C++ standard, warnings, optimization, debug information, and sanitizers.

## Commands

- castle new <name> creates a project.
- castle init [name] initializes the current directory.
- castle build compiles and links the project.
- castle run [args] builds and runs the binary.
- castle check runs a syntax-only check.
- castle test builds and runs files under tests/.
- castle clean removes build output and compile_commands.json.
- castle fmt and castle tidy run the configured Clang tools.

The Clang backend is the default. A CMake backend is available when a project needs it. Build flags can override the manifest for one invocation.

## Development

```sh
cargo test
cargo run -- new example
```

See the source and the example project for the complete manifest and command behavior.

## License

MIT. See LICENSE.
