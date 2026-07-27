//! Terminal output helpers.
//!
//! A tiny [`Printer`] that wraps every message in a `Castle:` prefix and adds
//! ANSI color when it is safe to do so. Color is disabled when:
//! - the `NO_COLOR` environment variable is set (<https://no-color.org/>),
//! - the `CI` environment variable is set (avoid escape codes in logs),
//! - `TERM=dumb`, or
//! - the target stream is not a TTY.

use std::io::{self, IsTerminal};
use std::sync::OnceLock;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";

/// The Castle voice. Construct one and pass it around; it is zero-cost to clone
/// state-wise (there is none) but kept as a struct so commands stay uniform.
#[derive(Debug, Default, Clone, Copy)]
pub struct Printer;

impl Printer {
    /// Construct a new [`Printer`].
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Wrap `msg` in the given ANSI codes when color is enabled for `err`.
    fn paint(&self, err: bool, codes: &[&str], msg: &str) -> String {
        if !want_color(err) {
            return msg.to_string();
        }
        let mut s = String::with_capacity(msg.len() + 16);
        for c in codes {
            s.push_str(c);
        }
        s.push_str(msg);
        s.push_str(RESET);
        s
    }

    /// A build step in progress, e.g. `Castle: clang compile src/main.cpp`.
    pub fn step(&self, label: &str) {
        let prefix = self.paint(true, &[BOLD, CYAN], "Castle:");
        eprintln!("{prefix} {label}");
    }

    /// A successful outcome, printed to stdout.
    pub fn success(&self, msg: &str) {
        let prefix = self.paint(false, &[BOLD, GREEN], "Castle:");
        println!("{prefix} {msg}");
    }

    /// A neutral informational message, printed to stdout.
    pub fn info(&self, msg: &str) {
        let prefix = self.paint(false, &[BOLD], "Castle:");
        println!("{prefix} {msg}");
    }

    /// A warning, printed to stderr.
    pub fn warn(&self, msg: &str) {
        let prefix = self.paint(true, &[BOLD, YELLOW], "Castle:");
        let label = self.paint(true, &[YELLOW], "warning:");
        eprintln!("{prefix} {label} {msg}");
    }

    /// An error, printed to stderr.
    pub fn error(&self, msg: &str) {
        let prefix = self.paint(true, &[BOLD, RED], "Castle:");
        eprintln!("{prefix} {msg}");
    }

    /// A dimmed hint line (Cargo-style), printed to stderr.
    pub fn hint(&self, msg: &str) {
        let label = self.paint(true, &[DIM], "hint:");
        eprintln!("  {label} {msg}");
    }
}

fn want_color(err: bool) -> bool {
    static STDOUT: OnceLock<bool> = OnceLock::new();
    static STDERR: OnceLock<bool> = OnceLock::new();
    let slot: &'static OnceLock<bool> = if err { &STDERR } else { &STDOUT };
    *slot.get_or_init(|| {
        if std::env::var_os("NO_COLOR").is_some() {
            return false;
        }
        if std::env::var_os("CI").is_some() {
            return false;
        }
        if let Some(term) = std::env::var_os("TERM")
            && term == "dumb"
        {
            return false;
        }
        if err {
            io::stderr().is_terminal()
        } else {
            io::stdout().is_terminal()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printer_constructs() {
        let _p = Printer::new();
    }

    #[test]
    fn paint_returns_the_message() {
        // `paint` always returns the message (with or without escape codes);
        // we only assert the input survives, without touching process env
        // (which is unsafe to mutate in edition 2024 and racy under OnceLock).
        let p = Printer::new();
        let out = p.paint(false, &[BOLD], "castle");
        assert!(out.contains("castle"));
    }
}
