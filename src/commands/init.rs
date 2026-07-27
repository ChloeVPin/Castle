//! `castle init [name]` — scaffold a project in the current directory.

use crate::commands::common::validate_project_name;
use crate::error::{CastleError, Result};
use crate::output::Printer;

/// Scaffold into `cwd`. `name` defaults to the current directory's name.
pub fn run(name: Option<&str>, cmake: bool, printer: &Printer) -> Result<()> {
    let cwd = std::env::current_dir()?;
    let name = match name {
        Some(n) => n.to_string(),
        None => cwd
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                CastleError::InvalidProjectName(
                    "could not derive a project name from the current directory".to_string(),
                )
            })?
            .to_string(),
    };
    validate_project_name(&name)?;

    // Refuse to clobber an existing project.
    if cwd.join("castle.toml").is_file() || cwd.join("CMakeLists.txt").is_file() {
        return Err(CastleError::ProjectExists(cwd));
    }
    if cwd.join("src").join("main.cpp").is_file() {
        return Err(CastleError::ProjectExists(cwd.join("src/main.cpp")));
    }

    crate::commands::new::scaffold(&cwd, &name, cmake)?;
    printer.success(&format!(
        "initialized project `{name}` in current directory"
    ));
    printer.hint("run `castle build` to compile");
    Ok(())
}
