//! Project root discovery and source enumeration.
//!
//! Castle finds the project root by walking up from the current directory until
//! it sees a `castle.toml` *or* a `CMakeLists.txt` (backward compatible with
//! the original 0.1.0 design). Sources are every `*.cpp` under `src/`.

use std::path::{Path, PathBuf};

use crate::error::{CastleError, Result};

/// A resolved Castle project: its root directory and parsed manifest (if any).
#[derive(Debug, Clone)]
pub struct Project {
    /// Absolute path to the project root (the dir containing `castle.toml` or
    /// `CMakeLists.txt`).
    pub root: PathBuf,
    /// The manifest, or `None` if the project has no `castle.toml`.
    pub manifest: Option<crate::manifest::Manifest>,
}

impl Project {
    /// Discover the project by walking up from `cwd`.
    pub fn discover() -> Result<Self> {
        let root = find_root_dir().ok_or(CastleError::NotInProject)?;
        let manifest = crate::manifest::load(&root)?;
        Ok(Self { root, manifest })
    }

    /// The directory where build artifacts live (`<root>/build`).
    #[must_use]
    pub fn build_dir(&self) -> PathBuf {
        self.root.join("build")
    }

    /// Enumerate every `*.cpp` under `<root>/src`, sorted for deterministic builds.
    pub fn sources(&self) -> Result<Vec<PathBuf>> {
        collect_sources(&self.root.join("src"))
    }
}

/// Walk up from the current directory looking for a `castle.toml` or
/// `CMakeLists.txt` marker.
pub fn find_root_dir() -> Option<PathBuf> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if dir.join("castle.toml").is_file() || dir.join("CMakeLists.txt").is_file() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Recursively collect `*.cpp` files under `dir`, sorted.
pub fn collect_sources(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    visit(dir, &mut out)?;
    out.sort();
    if out.is_empty() {
        return Err(CastleError::NoSources {
            dir: dir.display().to_string(),
        });
    }
    Ok(out)
}

fn visit(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit(&path, out)?;
        } else if path.extension().is_some_and(|e| e == "cpp") {
            out.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn collect_sources_walks_recursively_and_sorts() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("src/util")).unwrap();
        fs::write(root.join("src/z.cpp"), b"").unwrap();
        fs::write(root.join("src/a.cpp"), b"").unwrap();
        fs::write(root.join("src/util/m.cpp"), b"").unwrap();
        fs::write(root.join("src/readme.md"), b"").unwrap(); // ignored

        let got = collect_sources(&root.join("src")).unwrap();
        let names: Vec<_> = got
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();
        assert_eq!(names, vec!["a.cpp", "m.cpp", "z.cpp"]);
    }

    #[test]
    fn collect_sources_errors_when_empty() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        let err = collect_sources(&tmp.path().join("src"));
        assert!(err.is_err());
    }
}
