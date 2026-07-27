//! `castle fmt` — run `clang-format` over the project's sources.
//!
//! If no `.clang-format` exists, Castle drops its default (see
//! [`crate::templates::CLANG_FORMAT`]) so the first `fmt` is opinionated but
//! predictable. Sources under `src/` and `tests/` are reformatted in place.

use std::path::PathBuf;
use std::process::Command;

use crate::error::{CastleError, Result};
use crate::output::Printer;
use crate::project::Project;
use crate::templates::CLANG_FORMAT;
use crate::toolchain::on_path;

/// Run `clang-format -i` over every `*.cpp`/`*.hpp` under `src/` and `tests/`.
pub fn run(printer: &Printer) -> Result<()> {
    if !on_path("clang-format") {
        return Err(CastleError::ToolMissing {
            tool: "clang-format",
        });
    }
    let project = Project::discover()?;

    // Seed a default .clang-format if the project lacks one.
    let cfg = project.root.join(".clang-format");
    if !cfg.is_file() {
        std::fs::write(&cfg, CLANG_FORMAT)?;
        printer.info("wrote default .clang-format");
    }

    let mut files = Vec::new();
    for sub in ["src", "tests"] {
        let dir = project.root.join(sub);
        if dir.is_dir() {
            collect(&dir, &mut files);
        }
    }
    if files.is_empty() {
        printer.warn("no source files to format");
        return Ok(());
    }

    let mut cmd = Command::new("clang-format");
    cmd.arg("-i").args(&files);
    crate::shell::run_step(
        &mut cmd,
        &format!("clang-format {} file(s)", files.len()),
        printer,
    )?;
    printer.success("format finished");
    Ok(())
}

fn collect(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in rd.flatten() {
        let p = entry.path();
        if p.is_dir() {
            collect(&p, out);
        } else if p
            .extension()
            .is_some_and(|e| e == "cpp" || e == "hpp" || e == "h")
        {
            out.push(p);
        }
    }
}
