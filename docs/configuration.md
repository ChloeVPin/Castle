# Configuration

Castle uses an optional `castle.toml` file at the project root. Commands work with defaults when the file is absent, while `castle new` and `castle init` generate a commented manifest.

CLI flags take precedence over manifest values for the command being run.

## Example

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
sanitizer = "address"
```

## Top-level fields

### `backend`

Selects the build backend.

- `"clang"` — direct `clang++` compilation and linking. This is the default.
- `"cmake"` — generated CMake project configured and built by CMake.

Override it for one build with:

```bash
castle build --backend cmake
```

## `[package]`

### `name`

The output binary name. Scaffolded projects use the project name.

### `version`

An informational project version.

### `cxx_standard`

Supported values are `20`, `23`, and `26`. Castle checks that the detected Clang version is new enough for the selected standard.

Override it for one command with:

```bash
castle build --cxx-standard=26
```

## `[profile]`

### `opt_level`

Optimization level passed to the compiler. Supported values are `"0"`, `"1"`, `"2"`, `"3"`, and `"s"`.

### `debug_info`

When `true`, Castle includes debug information for non-release builds.

### `warnings_as_errors`

When `true`, compiler warnings are treated as errors.

### `pedantic`

When `true`, Castle enables `-Wpedantic`. The CLI `--pedantic` flag forces this setting on for the current build; it does not mean warnings-as-errors.

### `sanitizer`

Optional sanitizer preset. Supported values are:

- `"address"`
- `"thread"`
- `"undefined"`
- `"memory"`

The equivalent CLI flags are `--asan`, `--tsan`, `--ubsan`, and `--msan`.

## Build profiles

Debug behavior is the default. `castle build --release` selects the release profile behavior for the current command.

Castle derives compiler and linker flags in one place and shares them between the direct Clang and CMake backends so backend selection does not create a second configuration model.

## clangd and compile commands

A successful build writes or mirrors `compile_commands.json` to the project root. clangd-compatible editors can consume that file directly.
