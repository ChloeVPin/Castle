//! `castle clean` — remove build artifacts.

use std::io::{IsTerminal, Write};

use crate::error::Result;
use crate::output::Printer;
use crate::project::Project;

/// Remove `build/` and `compile_commands.json`. Prompts unless `force`.
pub fn run(force: bool, printer: &Printer) -> Result<()> {
    let project = Project::discover()?;
    let build = project.build_dir();
    let cc = project.root.join("compile_commands.json");

    let remove = if force {
        true
    } else if !std::io::stdin().is_terminal() {
        // Non-interactive (piped stdin / CI): refuse without --force to avoid surprises.
        printer.warn("refusing to clean in a non-interactive context without --force");
        return Ok(());
    } else {
        print!(
            "Castle: remove {} and {}? [y/N] ",
            build.display(),
            cc.display()
        );
        let _ = std::io::stdout().flush();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;
        matches!(buf.trim().to_ascii_lowercase().as_str(), "y" | "yes")
    };

    if !remove {
        printer.info("clean cancelled");
        return Ok(());
    }

    if build.exists() {
        std::fs::remove_dir_all(&build)?;
        printer.info(&format!("removed {}", build.display()));
    }
    if cc.exists() {
        std::fs::remove_file(&cc)?;
        printer.info(&format!("removed {}", cc.display()));
    }
    printer.success("clean finished");
    Ok(())
}
