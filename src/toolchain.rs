//! Toolchain detection: clang version probing and `PATH` lookups.

use std::process::Command;

use crate::error::{CastleError, Result};

/// Information about the located `clang`.
#[derive(Debug, Clone)]
pub struct ClangInfo {
    /// The major version number, e.g. `21` for `clang 21.0.0`.
    pub major: u32,
    /// The full `clang --version` stdout, for display.
    pub raw: String,
}

/// Run `clang --version` and parse the major version.
pub fn detect_clang() -> Result<ClangInfo> {
    let output = Command::new("clang")
        .arg("--version")
        .output()
        .map_err(|_| CastleError::ClangNotFound)?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let major = parse_major(&stdout)
        .ok_or_else(|| CastleError::ClangVersionParse(stdout.trim().to_string()))?;
    Ok(ClangInfo { major, raw: stdout })
}

/// Like [`detect_clang`] but enforce a minimum major version, and also verify
/// that the C++ driver `clang++` is on `PATH` (Castle compiles and links via
/// `clang++`, not `clang`, so that the C++ standard library is linked
/// automatically).
pub fn require_clang(min_major: u32) -> Result<ClangInfo> {
    let info = detect_clang()?;
    if info.major < min_major {
        return Err(CastleError::ClangTooOld {
            found: info.major,
            required: min_major,
        });
    }
    if !on_path("clang++") {
        return Err(CastleError::ToolMissing { tool: "clang++" });
    }
    Ok(info)
}

/// Parse the major version out of a `clang --version`-style string.
///
/// Handles both upstream (`clang version 18.1.0`) and Apple
/// (`Apple clang version 21.0.0`) layouts by grabbing the numeric token that
/// follows the word `version`.
pub fn parse_major(stdout: &str) -> Option<u32> {
    let first = stdout.lines().next()?;
    let mut saw_version = false;
    for tok in first.split_whitespace() {
        if saw_version {
            let digits: String = tok.chars().take_while(|c| c.is_ascii_digit()).collect();
            return digits.parse::<u32>().ok().filter(|&n| n > 0);
        }
        if tok.eq_ignore_ascii_case("version") {
            saw_version = true;
        }
    }
    None
}

/// Return `true` if `tool` exists as a file somewhere on `PATH`.
///
/// On Windows most binaries carry a `.exe` extension on disk even when
/// `Command::new("clang")` resolves correctly (the OS adds the extension
/// during process creation).  We therefore probe both the bare name and the
/// `.exe`-suffixed variant so `on_path` mirrors what the OS can actually
/// spawn.
pub fn on_path(tool: &str) -> bool {
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    let exe = format!("{tool}.exe");
    std::env::split_paths(&path).any(|dir| dir.join(tool).is_file() || dir.join(&exe).is_file())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_upstream_clang_version() {
        assert_eq!(
            parse_major("clang version 18.1.0\nTarget: x86_64\n"),
            Some(18)
        );
    }

    #[test]
    fn parses_apple_clang_version() {
        assert_eq!(
            parse_major("Apple clang version 21.0.0 (clang-2100.3.25.1)\n"),
            Some(21)
        );
    }

    #[test]
    fn rejects_unparseable_output() {
        assert_eq!(parse_major("not a version string"), None);
    }
}
