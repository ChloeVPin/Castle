//! `castle tidy` — run `clang-tidy` over the project's sources.
//!
//! Requires a prior `castle build` so that `compile_commands.json` exists at
//! the project root (Castle writes one for both backends). clang-tidy uses it
//! to apply the project's real compile flags.

use std::process::Command;

use crate::error::{CastleError, Result};
use crate::output::Printer;
use crate::project::Project;
use crate::templates::CLANG_TIDY;
use crate::toolchain::on_path;

/// Run `clang-tidy` over every `*.cpp` under `src/`.
pub fn run(printer: &Printer) -> Result<()> {
    if !on_path("clang-tidy") {
        return Err(CastleError::ToolMissing { tool: "clang-tidy" });
    }
    let project = Project::discover()?;

    let cc = project.root.join("compile_commands.json");
    if !cc.is_file() {
        return Err(CastleError::NoCompileCommands);
    }

    // Seed a default .clang-tidy if the project lacks one.
    let cfg = project.root.join(".clang-tidy");
    if !cfg.is_file() {
        std::fs::write(&cfg, CLANG_TIDY)?;
        printer.info("wrote default .clang-tidy");
    }

    let sources = project.sources()?;
    let mut cmd = Command::new("clang-tidy");
    cmd.arg("-p").arg(&project.root).args(&sources);
    crate::shell::run_step(
        &mut cmd,
        &format!("clang-tidy {} source(s)", sources.len()),
        printer,
    )?;
    printer.success("tidy finished");
    Ok(())
}
