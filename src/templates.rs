//! Embedded scaffolding templates.
//!
//! Every template ships as a `.txt` file under `src/templates/` and is baked
//! into the binary with [`include_str!`] — the original Castle's loveliest
//! trick. [`render`] does minimal `{{KEY}}` substitution at scaffold time.

/// Default `src/main.cpp` for a new project (C++23 `std::println`).
pub const CPP_HELLO_WORLD: &str = include_str!("templates/cpp_hello_world.txt");
/// Fallback `.clangd` config (compile_commands.json overrides at build time).
pub const CLANGD: &str = include_str!("templates/clangd.txt");
/// `CMakeLists.txt` for the opt-in `cmake` backend.
pub const CMAKE_LISTS: &str = include_str!("templates/cmake_lists.txt");
/// Default `.clang-format` applied by `castle fmt`.
pub const CLANG_FORMAT: &str = include_str!("templates/clang_format.txt");
/// Default `.clang-tidy` checks applied by `castle tidy`.
pub const CLANG_TIDY: &str = include_str!("templates/clang_tidy.txt");
/// Default `tests/main.cpp` for a new project.
pub const TEST_MAIN: &str = include_str!("templates/test_main.txt");
/// Default `.gitignore` for a new project.
pub const GITIGNORE: &str = include_str!("templates/gitignore.txt");
/// Default `castle.toml` manifest for a new project.
pub const CASTLE_TOML: &str = include_str!("templates/castle_toml.txt");

/// Substitute every `{{KEY}}` in `tmpl` with its paired value.
#[must_use]
pub fn render(tmpl: &str, pairs: &[(&str, &str)]) -> String {
    let mut out = tmpl.to_string();
    for (key, val) in pairs {
        out = out.replace(&format!("{{{{{key}}}}}"), val);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_substitutes_placeholders() {
        let out = render("hello {{WHO}}!", &[("WHO", "castle")]);
        assert_eq!(out, "hello castle!");
    }

    #[test]
    fn render_leaves_unknown_placeholders_intact() {
        let out = render("{{A}} {{B}}", &[("A", "x")]);
        assert_eq!(out, "x {{B}}");
    }

    #[test]
    fn all_templates_are_embedded() {
        // Smoke test: every constant is non-empty.
        for s in [
            CPP_HELLO_WORLD,
            CLANGD,
            CMAKE_LISTS,
            CLANG_FORMAT,
            CLANG_TIDY,
            TEST_MAIN,
            GITIGNORE,
            CASTLE_TOML,
        ] {
            assert!(!s.is_empty());
        }
    }
}
