# Architecture

A short tour of how Castle is put together. The whole thing is meant to be
readable in one sitting.

## Module map

```
src/
├── main.rs            # thin: parse CLI, dispatch, print errors + hints, ExitCode
├── cli.rs             # clap derive definitions (subcommands + flags)
├── error.rs           # CastleError enum (thiserror) + Result alias
├── output.rs          # Printer: colored, NO_COLOR-aware "Castle:" voice
├── toolchain.rs       # clang version parsing, clang++/tool PATH lookups
├── manifest.rs        # castle.toml types (serde) + defaults + load()
├── templates.rs       # include_str! templates + {{PLACEHOLDER}} renderer
├── project.rs         # walk-up root discovery + src/*.cpp enumeration
├── backend.rs         # Flags derivation + clang/cmake backends + compile_commands.json
├── shell.rs           # run_step(): labeled subprocess with inherited stdio
├── commands/
│   ├── mod.rs         # re-exports
│   ├── common.rs      # resolve_build_config(): merge manifest + CLI overrides
│   ├── new.rs         # castle new (scaffold in a new dir)
│   ├── init.rs        # castle init (scaffold in cwd)
│   ├── build.rs       # castle build
│   ├── run.rs         # castle run (+ --watch via notify)
│   ├── check.rs       # castle check (-fsyntax-only)
│   ├── clean.rs       # castle clean
│   ├── test.rs        # castle test (build + run tests/)
│   ├── fmt.rs         # castle fmt (clang-format)
│   ├── tidy.rs        # castle tidy (clang-tidy)
│   └── version.rs     # castle version
└── templates/         # .txt scaffolding files, baked in with include_str!
    ├── cpp_hello_world.txt
    ├── test_main.txt
    ├── castle_toml.txt
    ├── cmake_lists.txt
    ├── clangd.txt
    ├── clang_format.txt
    ├── clang_tidy.txt
    └── gitignore.txt
```

## Control flow

```
                         ┌──────────┐
                         │  main.rs │  clap::parse → dispatch → ExitCode
                         └────┬─────┘
                              │
   ┌──────────────┬───────────┼────────────┬──────────────┐
   ▼              ▼           ▼            ▼              ▼
 new/init       build        run          check         clean/test/fmt/tidy
   │              │           │            │
   │              ▼           │            │
   │     Project::discover()  │            │
   │     (walk up for         │            │
   │      castle.toml OR      │            │
   │      CMakeLists.txt)     │            │
   │              │           │            │
   │              ▼           │            │
   │   resolve_build_config() │            │
   │   (manifest ◀── CLI)     │            │
   │              │           │            │
   │              ▼           │            │
   │      Backend::build ──┐  │            │
   │                       │  │            │
   │              ┌────────┴───┴──┐        │
   │              ▼               ▼        │
   │        clang backend    cmake backend │
   │        (clang++ -c      (generate     │
   │         per .cpp,       CMakeLists,   │
   │         then link)      cmake -S/-B)  │
   │              │               │        │
   │              └──────┬────────┘        │
   │                     ▼                 │
   │          compile_commands.json         │
   │          (at project root)             │
   │                     │                 │
   └─────────────────────┴─────────────────┘
                         │
              find_built_binary() ──▶ execute
```

### `castle new <name>`

1. `validate_project_name` (ASCII, no spaces, single dir).
2. Refuse if the directory already exists.
3. `scaffold()` writes `src/main.cpp`, `tests/main.cpp`, `.clangd`,
   `.clang-format`, `.clang-tidy`, `.gitignore`, and `castle.toml` from the
   `include_str!` templates (with `{{PROJECT_NAME}}` rendered). For `--cmake`,
   also writes a `CMakeLists.txt` and flips the manifest's `backend` line.

### `castle build`

1. `Project::discover()` — walk up from `cwd` to find `castle.toml` or
   `CMakeLists.txt`; load the manifest if present.
2. `resolve_build_config()` — merge `[profile]` with CLI overrides
   (`--release`, `--cxx-standard`, sanitizer flags, `--pedantic`, `--backend`)
   into a `(Backend, Flags)` pair.
3. `require_clang(min)` — verify `clang` major version and that `clang++` is on
   `PATH`.
4. `Backend::build`:
   - **clang**: `clang++ -c` each `src/**/*.cpp` → `build/.../*.o`, then
     `clang++ *.o -o build/<name>`. Write `compile_commands.json` at the root.
   - **cmake**: render `CMakeLists.txt` from the manifest, `cmake -S . -B build`,
     `cmake --build build`, then mirror `build/compile_commands.json` to the root.

### `castle run`

1. Run `castle build` (same path as above).
2. `find_built_binary()` probes `build/<name>`, `build/bin`, `build/bin/<name>`.
3. Execute the binary with inherited stdio; pass through any args after `--`.
4. With `--watch`: set up a `notify` watcher on `src/` (debounced), rebuild and
   re-run on each change.

### `castle check`

Same flag resolution as `build`, but each source is checked with
`clang++ -fsyntax-only` — no object files, no link.

## Flag derivation (`backend::Flags`)

Both backends derive flags from one place so they stay in lockstep:

```
Profile (manifest [profile])  ──┐
CLI overrides (--release,     ──┤──▶ Flags { std_flag, opt_flag, debug,
  --cxx-standard, --asan, …)    │       pedantic, werror, sanitize }
                               │
cxx_standard (manifest or CLI) ─┘
```

`Flags::all()` produces the flat `clang++` argument list: `-std=c++NN -Ox -g
-Wall -Wextra [-Wpedantic] [-Werror] [-fsanitize=...]`.

## The `include_str!` template pattern

Every scaffolded file ships as a `.txt` under `src/templates/` and is baked
into the binary at compile time — no runtime file reads, no separate asset
directory. `templates::render()` does minimal `{{KEY}}` substitution at
scaffold time. Adding a new template is a two-step dance: drop a `.txt` file
into `src/templates/`, add a `pub const` in `templates.rs`.
