//! `castle test` — build and run the project's tests.
//!
//! Convention: every `*.cpp` under `tests/` is compiled together into a single
//! test binary (`build/castle_tests`) and executed. The scaffolded
//! `tests/main.cpp` uses plain `assert` so there is no framework dependency.

use std::path::PathBuf;
use std::process::Command;

use crate::commands::common::{min_clang_for, resolve_build_config};
use crate::error::{CastleError, Result};
use crate::output::Printer;
use crate::project::Project;
use crate::shell;
use crate::toolchain::require_clang;

/// Build and run the test binary.
pub fn run(cxx_override: Option<u32>, printer: &Printer) -> Result<()> {
    let project = Project::discover()?;
    let (_backend, flags) = resolve_build_config(&project, None, cxx_override, None, None, None)?;

    let min = min_clang_for(
        flags
            .std_flag
            .strip_prefix("-std=c++")
            .and_then(|s| s.parse().ok())
            .unwrap_or(23),
    );
    let _clang = require_clang(min)?;

    let tests_dir = project.root.join("tests");
    if !tests_dir.is_dir() {
        printer.warn("no tests/ directory found — nothing to test");
        return Ok(());
    }
    let mut sources = Vec::new();
    collect(&tests_dir, &mut sources)?;
    if sources.is_empty() {
        printer.warn("tests/ exists but contains no .cpp files — nothing to test");
        return Ok(());
    }
    sources.sort();

    let build = project.build_dir();
    std::fs::create_dir_all(&build)?;
    let bin = build.join(format!("castle_tests{}", std::env::consts::EXE_SUFFIX));

    let mut cmd = Command::new("clang++");
    cmd.args(flags.all()).args(&sources).arg("-o").arg(&bin);
    shell::run_step(
        &mut cmd,
        &format!("compile+link {} test source(s)", sources.len()),
        printer,
    )?;

    printer.step("run tests");
    match Command::new(&bin).status() {
        Ok(status) if status.success() => {
            printer.success("tests passed");
            Ok(())
        }
        Ok(status) => Err(CastleError::BackendFailed {
            label: "tests".to_string(),
            code: status.code().unwrap_or(1),
        }),
        Err(e) => Err(CastleError::Io(e)),
    }
}

fn collect(dir: &std::path::Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            collect(&p, out)?;
        } else if p.extension().is_some_and(|e| e == "cpp") {
            out.push(p);
        }
    }
    Ok(())
}
