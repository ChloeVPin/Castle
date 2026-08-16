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

    let min = min_clang_for(
        flags
            .std_flag
            .strip_prefix("-std=c++")
            .and_then(|s| s.parse().ok())
            .unwrap_or(23),
    );
    let clang = require_clang(min)?;
    let profile = opts.profile.unwrap_or("debug");
    let standard = flags.std_flag.trim_start_matches("-std=");
    printer.info(&format!(
        "clang {}, {standard}, {profile}, {} backend",
        clang.major,
        backend_label(backend)
    ));

    backend.build(&project, &flags, printer)?;

    if crate::toolchain::on_path("ccache") && backend == Backend::Clang {
        printer.hint(
            "ccache detected; use the CMake backend with CMAKE_CXX_COMPILER_LAUNCHER to enable it",
        );
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
