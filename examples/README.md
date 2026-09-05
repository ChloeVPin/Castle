# Castle examples

Two small projects you can build with Castle right now. They live outside the
Castle source tree so they don't interfere with `cargo test`.

## `hello`: a single-file binary

The default scaffold. Reproduce with:

```bash
castle new hello
cd hello
castle build
castle run
```

`hello/src/main.cpp`:

```cpp
#include <print>

int main() {
    std::println("Hello, world! Your castle stands.");
    return 0;
}
```

Try the extras:

```bash
castle check            # fast syntax check
castle test             # build & run tests/main.cpp
castle fmt              # format with the seeded .clang-format
castle run --watch      # edit src/main.cpp and watch it rebuild
castle build --release  # optimized build
castle build --asan     # AddressSanitizer build
castle clean --force    # start over
```

## `multi`: a multi-file project using the cmake backend

A two-file project that shows the opt-in CMake backend and a shared header.

Reproduce with:

```bash
castle new multi --cmake
cd multi
# add the files below, then:
castle build --backend cmake
castle run
```

`multi/src/greet.hpp`:

```cpp
#pragma once
#include <string>
#include <print>

inline void greet(const std::string& who) {
    std::println("Hello, {}! Your castle stands.", who);
}
```

`multi/src/main.cpp`:

```cpp
#include "greet.hpp"

int main() {
    greet("multi-file castle");
    return 0;
}
```

The generated `CMakeLists.txt` globs `src/**/*.cpp`, so adding `greet.cpp`
later requires no manifest edits: just `castle build` again.

## Why these matter

- `hello` proves the zero-config, single-command story: `new, build, run`,
  with `compile_commands.json` landing for clangd automatically.
- `multi` shows the escape hatch: when you outgrow the minimal clang backend,
  flip `backend = "cmake"` (or pass `--backend cmake`) and you get a real
  CMake project without leaving Castle's CLI.
