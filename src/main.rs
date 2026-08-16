//! Castle — a minimal Cargo-inspired C++ project manager.
//!
//! Castle scaffolds, builds, and runs C++ projects with clang (≥ 18) and
//! C++23 by default. See [`cli`] for the command surface and [`backend`] for
//! the clang/cmake build backends.
//!
//! Originally ~200 lines written in two days as a learning milestone; this
//! expansion keeps that readable, hackable spirit while adding essence and
//! best practice.

mod backend;
mod cli;
mod commands;
mod error;
mod manifest;
mod output;
mod project;
mod shell;
mod templates;
mod toolchain;

use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, Command};

use crate::output::Printer;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let printer = Printer::new();
    match dispatch(&cli.command, &printer) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            printer.error(&e.to_string());
            hint_for(&e, &printer);
            ExitCode::from(exit_code_for(&e))
        }
    }
}

/// Route a parsed command to its handler.
fn dispatch(cmd: &Command, printer: &Printer) -> error::Result<()> {
    match cmd {
        Command::New { name, cmake } => commands::run_new(name, *cmake, printer),
        Command::Init { name, cmake } => commands::run_init(name.as_deref(), *cmake, printer),
        Command::Build {
            release,
            cxx_standard,
            backend,
            pedantic,
            ..
        } => commands::run_build(
            commands::build::BuildOpts {
                profile: if *release { Some("release") } else { None },
                cxx_standard: *cxx_standard,
                backend: backend.as_deref(),
                sanitizer: cmd.sanitizer_override(),
                // `--pedantic` only forces ON; absence preserves the manifest default.
                pedantic: if *pedantic { Some(true) } else { None },
            },
            printer,
        ),
        Command::Run {
            release,
            cxx_standard,
            backend,
            watch,
            args,
            ..
        } => commands::run_run(
            commands::run::RunOpts {
                profile: if *release { Some("release") } else { None },
                cxx_standard: *cxx_standard,
                backend: backend.as_deref(),
                sanitizer: cmd.sanitizer_override(),
                watch: *watch,
                args,
            },
            printer,
        ),
        Command::Check { cxx_standard } => commands::run_check(*cxx_standard, printer),
        Command::Clean { force } => commands::run_clean(*force, printer),
        Command::Test { cxx_standard } => commands::run_test(*cxx_standard, printer),
        Command::Fmt => commands::run_fmt(printer),
        Command::Tidy => commands::run_tidy(printer),
        Command::Version => commands::run_version(printer),
    }
}

/// Print a contextual hint for some errors (Cargo-style).
fn hint_for(e: &error::CastleError, printer: &Printer) {
    use error::CastleError;
    match e {
        CastleError::NotInProject => {
            printer.hint("run `castle new <name>` to create one, or `cd` into a project");
        }
        CastleError::NoCompileCommands => {
            printer.hint("run `castle build` first so clang-tidy can use your flags");
        }
        CastleError::ToolMissing { tool } if *tool == "cmake" => {
            printer.hint("or use the default clang backend: `castle build --backend clang`");
        }
        _ => {}
    }
}

/// Map an error to a process exit code.
fn exit_code_for(e: &error::CastleError) -> u8 {
    use error::CastleError;
    match e {
        CastleError::BackendFailed { code, .. } => (*code).clamp(1, 255) as u8,
        _ => 1,
    }
}
