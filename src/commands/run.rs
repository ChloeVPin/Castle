//! `castle run` — build and run, with optional watch mode and arg passthrough.

use std::process::Command;
use std::time::Duration;

use crate::backend::find_built_binary;
use crate::commands::build::{BuildOpts, run as run_build};
use crate::error::{CastleError, Result};
use crate::output::Printer;
use crate::project::Project;

/// Run options lifted from the CLI.
pub struct RunOpts<'a> {
    pub profile: Option<&'a str>,
    pub cxx_standard: Option<u32>,
    pub backend: Option<&'a str>,
    pub sanitizer: Option<&'a str>,
    pub watch: bool,
    pub args: &'a [String],
}

/// Build then run. With `--watch`, rebuild and re-run on source change.
pub fn run(opts: RunOpts<'_>, printer: &Printer) -> Result<()> {
    if opts.watch {
        return run_watch(opts, printer);
    }

    run_build(
        BuildOpts {
            profile: opts.profile,
            cxx_standard: opts.cxx_standard,
            backend: opts.backend,
            sanitizer: opts.sanitizer,
            pedantic: None,
        },
        printer,
    )?;

    let project = Project::discover()?;
    let name = project
        .manifest
        .as_ref()
        .map(|m| m.package.name.clone())
        .unwrap_or_else(|| "bin".to_string());

    let Some(bin) = find_built_binary(&project, &name) else {
        return Err(CastleError::BackendFailed {
            label: "locate binary".to_string(),
            code: 1,
        });
    };

    printer.step(&format!("run {}", bin.display()));
    let status = Command::new(&bin).args(opts.args).status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(CastleError::BackendFailed {
            label: "program".to_string(),
            code: s.code().unwrap_or(1),
        }),
        Err(e) => Err(CastleError::Io(e)),
    }
}

/// Watch `src/` for changes; on each change (debounced), rebuild and re-run.
fn run_watch(opts: RunOpts<'_>, printer: &Printer) -> Result<()> {
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};

    let project = Project::discover()?;
    let watch_dir = project.root.join("src");
    if !watch_dir.is_dir() {
        return Err(CastleError::NoSources {
            dir: watch_dir.display().to_string(),
        });
    }

    // Initial build + run.
    run_once(&opts, printer)?;

    let (tx, rx) = std::sync::mpsc::channel::<notify::Result<notify::Event>>();
    let mut watcher: RecommendedWatcher = notify::Watcher::new(
        move |res| {
            let _ = tx.send(res);
        },
        notify::Config::default().with_poll_interval(Duration::from_millis(200)),
    )?;
    watcher.watch(&watch_dir, RecursiveMode::Recursive)?;

    printer.info("watching src/ for changes (Ctrl-C to stop)");
    let mut last = std::time::Instant::now();
    for event in rx {
        match event {
            Ok(_e) => {
                // Debounce: ignore events within 150ms of the last.
                let now = std::time::Instant::now();
                if now.duration_since(last) < Duration::from_millis(150) {
                    continue;
                }
                last = now;
                printer.info("change detected — rebuilding");
                if let Err(e) = run_once(&opts, printer) {
                    printer.error(&e.to_string());
                }
            }
            Err(e) => {
                printer.warn(&format!("watcher: {e}"));
            }
        }
    }
    Ok(())
}

fn run_once(opts: &RunOpts<'_>, printer: &Printer) -> Result<()> {
    run_build(
        BuildOpts {
            profile: opts.profile,
            cxx_standard: opts.cxx_standard,
            backend: opts.backend,
            sanitizer: opts.sanitizer,
            pedantic: None,
        },
        printer,
    )?;
    let project = Project::discover()?;
    let name = project
        .manifest
        .as_ref()
        .map(|m| m.package.name.clone())
        .unwrap_or_else(|| "bin".to_string());
    if let Some(bin) = find_built_binary(&project, &name) {
        printer.step(&format!("run {}", bin.display()));
        let _ = Command::new(&bin).args(opts.args).status();
    }
    Ok(())
}
