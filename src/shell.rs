//! Subprocess helpers.
//!
//! [`run_step`] prints a labeled step via the [`Printer`], runs a command with
//! inherited stdio (so compiler diagnostics stream straight to the user), and
//! returns a typed error on non-zero exit.

use std::process::Command;

use crate::error::{CastleError, Result};
use crate::output::Printer;

/// Run `cmd`, labeling the step `label`. Errors map to [`CastleError::BackendFailed`].
pub fn run_step(cmd: &mut Command, label: &str, printer: &Printer) -> Result<()> {
    printer.step(label);
    match cmd.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(CastleError::BackendFailed {
            label: label.to_string(),
            code: status.code().unwrap_or(1),
        }),
        Err(e) => Err(CastleError::Io(e)),
    }
}
