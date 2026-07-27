//! `castle new <name>` — scaffold a project in a new directory.

use std::path::PathBuf;

use crate::commands::common::validate_project_name;
use crate::error::{CastleError, Result};
use crate::output::Printer;
use crate::templates::{
    CASTLE_TOML, CLANG_FORMAT, CLANG_TIDY, CLANGD, CPP_HELLO_WORLD, GITIGNORE, TEST_MAIN, render,
};

/// Scaffold a new project at `<name>/`.
pub fn run(name: &str, cmake: bool, printer: &Printer) -> Result<()> {
    validate_project_name(name)?;
    let dir = PathBuf::from(name);
    if dir.exists() {
        return Err(CastleError::ProjectExists(dir));
    }
    scaffold(&dir, name, cmake)?;
    printer.success(&format!("created project `{name}`"));
    printer.hint("cd into it and run `castle build`");
    Ok(())
}

/// Write the full project layout. Shared by `new` and `init`.
pub fn scaffold(dir: &std::path::Path, name: &str, cmake: bool) -> Result<()> {
    let src = dir.join("src");
    let tests = dir.join("tests");
    std::fs::create_dir_all(&src)?;
    std::fs::create_dir_all(&tests)?;

    std::fs::write(src.join("main.cpp"), CPP_HELLO_WORLD)?;
    std::fs::write(tests.join("main.cpp"), TEST_MAIN)?;
    std::fs::write(dir.join(".clangd"), CLANGD)?;
    std::fs::write(dir.join(".clang-format"), CLANG_FORMAT)?;
    std::fs::write(dir.join(".clang-tidy"), CLANG_TIDY)?;
    std::fs::write(dir.join(".gitignore"), GITIGNORE)?;

    // Manifest. The template ships a commented-out `# backend = "clang"` line;
    // for cmake projects we uncomment-and-switch it to `backend = "cmake"`.
    let toml_text = render(CASTLE_TOML, &[("PROJECT_NAME", name)]);
    let toml_text = if cmake {
        toml_text.replacen("# backend = \"clang\"", "backend = \"cmake\"", 1)
    } else {
        toml_text
    };
    std::fs::write(dir.join("castle.toml"), toml_text)?;

    // For cmake projects, also emit a CMakeLists.txt up front.
    if cmake {
        let cmake_lists = render(
            crate::templates::CMAKE_LISTS,
            &[("PROJECT_NAME", name), ("CXX_STANDARD", "23")],
        );
        std::fs::write(dir.join("CMakeLists.txt"), cmake_lists)?;
    }

    Ok(())
}
