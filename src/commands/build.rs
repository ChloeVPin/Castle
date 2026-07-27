//! `castle build` — compile and link the project.

use crate::backend::Backend;
use crate::commands::common::{min_clang_for, resolve_build_config};
use crate::error::Result;
use crate::output::Printer;
use crate::project::Project;
use crate::toolchain::require_clang;

/// Build options lifted from the CLI.
pub struct BuildOpts<'a> {
    pub profile: Option<&'a str>,
    pub cxx_standard: Option<u32>,
    pub backend: Option<&'a str>,
    pub sanitizer: Option<&'a str>,
    pub pedantic: Option<bool>,
}

/// Run a build with the given options.
pub fn run(opts: BuildOpts<'_>, printer: &Printer) -> Result<()> {
    let project = Project::discover()?;
    let (backend, flags) = resolve_build_config(
        &project,
        opts.profile,
        opts.cxx_standard,
        opts.sanitizer,
        opts.pedantic,
        opts.backend,
    )?;

    // Verify clang is new enough for the chosen standard.
    let min = min_clang_for(
        flags
            .std_flag
            .strip_prefix("-std=c++")
            .and_then(|s| s.parse().ok())
            .unwrap_or(23),
    );
    let clang = require_clang(min)?;
    let driver = if backend == Backend::Clang {
        "clang++"
    } else {
        "cmake"
    };
    printer.info(&format!(
        "using clang {} ({}, via {driver}), backend = {}",
        clang.major,
        flags.std_flag,
        backend_label(backend)
    ));

    backend.build(&project, &flags, printer)?;

    // ccache hint: Castle's clang backend doesn't yet route through ccache,
    // so point the user at the standard env var if it's installed.
    if crate::toolchain::on_path("ccache") && backend == Backend::Clang {
        printer.hint("ccache detected — set `CCACHE_PREFIX=ccache` (or use the cmake backend with CMAKE_CXX_COMPILER_LAUNCHER) to speed up rebuilds");
    }

    printer.success("build finished");
    Ok(())
}

fn backend_label(b: Backend) -> &'static str {
    match b {
        Backend::Clang => "clang",
        Backend::Cmake => "cmake",
    }
}
