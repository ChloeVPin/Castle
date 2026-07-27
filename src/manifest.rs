//! The `castle.toml` manifest.
//!
//! Parsed with `serde` + `toml`. Every field has a default, so a project with
//! no manifest (or a partial one) still builds. Scaffolding writes a fully
//! commented template (see [`crate::templates::CASTLE_TOML`]); this module only
//! owns parsing and defaults.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{CastleError, Result};

/// The top-level manifest.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Manifest {
    /// `[package]` metadata.
    pub package: Package,
    /// `[profile]` build settings.
    pub profile: Profile,
    /// Optional top-level `backend = "clang" | "cmake"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<String>,
}

/// `[package]` — identity and standard.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Package {
    /// The project (and output binary) name.
    pub name: String,
    /// Semantic version, informational.
    pub version: String,
    /// Default C++ standard (23).
    pub cxx_standard: u32,
}

/// `[profile]` — compiler flags derived from these.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Profile {
    /// Optimization level: `"0"`, `"1"`, `"2"`, `"3"`, `"s"`.
    pub opt_level: String,
    /// Emit `-g` debug info in non-release builds.
    pub debug_info: bool,
    /// Treat warnings as errors (`-Werror`).
    pub warnings_as_errors: bool,
    /// Enable `-Wpedantic`.
    pub pedantic: bool,
    /// Optional sanitizer: `"address"`, `"thread"`, `"undefined"`, `"memory"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sanitizer: Option<String>,
}

impl Default for Package {
    fn default() -> Self {
        Self {
            name: "bin".to_string(),
            version: "0.1.0".to_string(),
            cxx_standard: 23,
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            opt_level: "0".to_string(),
            debug_info: true,
            warnings_as_errors: false,
            pedantic: true,
            sanitizer: None,
        }
    }
}

/// Load a manifest from `dir/castle.toml`, or `Ok(None)` if absent.
pub fn load(dir: &Path) -> Result<Option<Manifest>> {
    let path = dir.join("castle.toml");
    if !path.is_file() {
        return Ok(None);
    }
    let text = std::fs::read_to_string(&path)?;
    let m: Manifest =
        toml::from_str(&text).map_err(|e| CastleError::ManifestParse(e.to_string()))?;
    Ok(Some(m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sensible() {
        let m = Manifest::default();
        assert_eq!(m.package.cxx_standard, 23);
        assert!(m.profile.pedantic);
        assert!(m.profile.debug_info);
        assert!(m.backend.is_none());
    }

    #[test]
    fn parses_full_manifest() {
        let toml = r#"
backend = "cmake"
[package]
name = "demo"
version = "0.2.0"
cxx_standard = 26
[profile]
opt_level = "2"
debug_info = false
warnings_as_errors = true
pedantic = false
sanitizer = "address"
"#;
        let m: Manifest = toml::from_str(toml).unwrap();
        assert_eq!(m.package.name, "demo");
        assert_eq!(m.package.cxx_standard, 26);
        assert_eq!(m.profile.opt_level, "2");
        assert_eq!(m.profile.sanitizer.as_deref(), Some("address"));
        assert_eq!(m.backend.as_deref(), Some("cmake"));
    }

    #[test]
    fn parses_minimal_manifest_with_defaults() {
        let toml = r#"
[package]
name = "x"
"#;
        let m: Manifest = toml::from_str(toml).unwrap();
        assert_eq!(m.package.name, "x");
        // defaults filled in
        assert_eq!(m.package.cxx_standard, 23);
        assert!(m.profile.pedantic);
    }
}
