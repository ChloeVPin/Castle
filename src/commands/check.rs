//! `castle check` — a fast type/syntax check pass with `-fsyntax-only`.
//!
//! We reuse the same flag derivation as `build` but skip linking. Each
//! translation unit is checked with the project's real flags (so warnings
//! behavior matches `build`), and we honor the `--cxx-standard` override.

use std::process::Command;

use crate::commands::common::{min_clang_for, resolve_build_config};
use crate::error::{CastleError, Result};
use crate::output::Printer;
use crate::project::Project;
use crate::shell;
use crate::toolchain::require_clang;

/// Run a syntax/type check over every source file.
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

    let sources = project.sources()?;
    let mut failed = 0usize;
    for src in &sources {
        let mut cmd = Command::new("clang++");
        cmd.args(flags.all()).arg("-fsyntax-only").arg(src);
        let label = format!("check {}", pretty_rel(&project.root, src));
        if shell::run_step(&mut cmd, &label, printer).is_err() {
            failed += 1;
        }
    }
    if failed == 0 {
        printer.success(&format!("{} source(s) checked, no errors", sources.len()));
        Ok(())
    } else {
        Err(CastleError::BackendFailed {
            label: format!("check ({failed} source(s) failed)"),
            code: 1,
        })
    }
}

fn pretty_rel(root: &std::path::Path, p: &std::path::Path) -> String {
    p.strip_prefix(root)
        .map(|r| r.display().to_string())
        .unwrap_or_else(|_| p.display().to_string())
}
