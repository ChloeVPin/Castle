//! Shared helpers for build-related subcommands.
//!
//! [`resolve_build_config`] merges the manifest's `[profile]` with CLI overrides
//! (`--release`, `--cxx-standard`, sanitizer flags, `--pedantic`, `--backend`)
//! into a concrete `(Backend, Flags)` pair. Keeping this in one place means
//! `build`, `run`, and `check` can never drift apart.

use crate::backend::{Backend, Flags};
use crate::error::{CastleError, Result};
use crate::project::Project;

/// Merge manifest defaults with CLI overrides.
///
/// - `profile_cli`: `"debug"` or `"release"`.
/// - `cxx_override`: e.g. `Some(26)`.
/// - `sanitizer_override`: e.g. `Some("address")` (already chosen by the CLI).
/// - `pedantic_override`: `--pedantic` on the CLI.
/// - `backend_override`: `--backend clang|cmake`.
#[allow(clippy::too_many_arguments)]
pub fn resolve_build_config(
    project: &Project,
    profile_cli: Option<&str>,
    cxx_override: Option<u32>,
    sanitizer_override: Option<&str>,
    pedantic_override: Option<bool>,
    backend_override: Option<&str>,
) -> Result<(Backend, Flags)> {
    let mut profile = project
        .manifest
        .as_ref()
        .map(|m| m.profile.clone())
        .unwrap_or_default();

    // Profile overlay: --release forces -O2 and drops -g; --debug is the default.
    if let Some(p) = profile_cli {
        match p {
            "release" => {
                profile.opt_level = "2".to_string();
                profile.debug_info = false;
            }
            "debug" => {
                profile.opt_level = "0".to_string();
                profile.debug_info = true;
            }
            other => {
                return Err(CastleError::InvalidValue {
                    field: "profile",
                    value: other.to_string(),
                });
            }
        }
    }

    if let Some(ped) = pedantic_override {
        profile.pedantic = ped;
    }

    // C++ standard: CLI override > manifest > default (23).
    let cxx = cxx_override
        .or(project.manifest.as_ref().map(|m| m.package.cxx_standard))
        .unwrap_or(23);

    let flags = Flags::from(&profile, cxx, sanitizer_override);
    let backend = Backend::resolve(project.manifest.as_ref(), backend_override)?;
    Ok((backend, flags))
}

/// The minimum clang major version Castle requires for a given standard.
#[must_use]
pub fn min_clang_for(cxx_standard: u32) -> u32 {
    match cxx_standard {
        n if n <= 20 => 14,
        23 => 18,
        26 => 19,
        _ => 19,
    }
}

/// Validate a project name: non-empty, ASCII, no spaces, no path separators.
pub fn validate_project_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(CastleError::InvalidProjectName(
            "project must have a name".to_string(),
        ));
    }
    if !name.is_ascii() || name.contains(' ') {
        return Err(CastleError::InvalidProjectName(
            "project can't contain spaces or non-ASCII characters".to_string(),
        ));
    }
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return Err(CastleError::InvalidProjectName(
            "project name must be a single directory name".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_rejects_bad_names() {
        assert!(validate_project_name("").is_err());
        assert!(validate_project_name("a b").is_err());
        assert!(validate_project_name("a/b").is_err());
        assert!(validate_project_name("a..b").is_err());
        assert!(validate_project_name("café").is_err());
        assert!(validate_project_name("good_name").is_ok());
    }

    #[test]
    fn min_clang_table_is_sane() {
        assert_eq!(min_clang_for(20), 14);
        assert_eq!(min_clang_for(23), 18);
        assert_eq!(min_clang_for(26), 19);
    }
}
