//! Build backends.
//!
//! Two interchangeable backends share one compiler-flag derivation (see
//! [`Flags`]):
//!
//! - [`Backend::Clang`] (default) — drives `clang` directly: one `-c` per
//!   translation unit into `build/`, then a final link. Needs only clang. This
//!   is what makes Castle runnable on a stock toolchain today.
//! - [`Backend::Cmake`] (opt-in via `castle.toml` or `--backend cmake`) —
//!   generates a `CMakeLists.txt` and shells out to `cmake`. Good for
//!   multi-file projects that want IDE integration and `compile_commands.json`
//!   for free.
//!
//! Both backends write a `compile_commands.json` to the project root so clangd
//! "just works" out of the box.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

use crate::error::{CastleError, Result};
use crate::manifest::{Manifest, Profile};
use crate::output::Printer;
use crate::project::Project;
use crate::shell;

/// Which backend to use for a given build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// Drive `clang` directly.
    Clang,
    /// Generate a CMake project and invoke `cmake`.
    Cmake,
}

impl Backend {
    /// Parse a backend name from a string.
    pub fn parse(s: &str) -> Result<Self> {
        match s {
            "clang" => Ok(Self::Clang),
            "cmake" => Ok(Self::Cmake),
            other => Err(CastleError::InvalidValue {
                field: "backend",
                value: other.to_string(),
            }),
        }
    }

    /// Pick the backend from: explicit `override_backend` (CLI) > manifest
    /// `backend` field > default [`Backend::Clang`].
    pub fn resolve(manifest: Option<&Manifest>, override_backend: Option<&str>) -> Result<Self> {
        if let Some(s) = override_backend {
            return Self::parse(s);
        }
        if let Some(m) = manifest
            && let Some(s) = &m.backend
        {
            return Self::parse(s);
        }
        Ok(Self::Clang)
    }

    /// Build the project end to end.
    pub fn build(self, project: &Project, flags: &Flags, printer: &Printer) -> Result<()> {
        match self {
            Self::Clang => clang_build(project, flags, printer),
            Self::Cmake => cmake_build(project, flags, printer),
        }
    }
}

/// A normalized set of compiler flags derived from the manifest profile and the
/// CLI profile overrides. Shared by both backends so they stay in lockstep.
#[derive(Debug, Clone)]
pub struct Flags {
    /// `-std=c++NN`.
    pub std_flag: String,
    /// Optimization: `-O0`, `-O1`, `-O2`, `-O3`, `-Os`.
    pub opt_flag: String,
    /// `-g` when present.
    pub debug: bool,
    /// `-Wpedantic` when present.
    pub pedantic: bool,
    /// `-Werror` when present.
    pub werror: bool,
    /// Sanitizer flag, e.g. `-fsanitize=address`.
    pub sanitize: Option<String>,
}

impl Flags {
    /// Build flags from a manifest profile plus a target standard and an
    /// optional CLI sanitizer override.
    #[must_use]
    pub fn from(profile: &Profile, cxx_standard: u32, sanitizer_override: Option<&str>) -> Self {
        let opt_flag = match profile.opt_level.as_str() {
            "0" => "-O0",
            "1" => "-O1",
            "2" => "-O2",
            "3" => "-O3",
            "s" => "-Os",
            _ => "-O0",
        }
        .to_string();

        let sanitize = sanitizer_override
            .or(profile.sanitizer.as_deref())
            .map(sanitize_flag);

        Self {
            std_flag: format!("-std=c++{cxx_standard}"),
            opt_flag,
            debug: profile.debug_info,
            pedantic: profile.pedantic,
            werror: profile.warnings_as_errors,
            sanitize,
        }
    }

    /// All flags as a flat `Vec<String>` suitable for `clang`.
    #[must_use]
    pub fn all(&self) -> Vec<String> {
        let mut v = vec![self.std_flag.clone(), self.opt_flag.clone()];
        if self.debug {
            v.push("-g".to_string());
        }
        v.push("-Wall".to_string());
        v.push("-Wextra".to_string());
        if self.pedantic {
            v.push("-Wpedantic".to_string());
        }
        if self.werror {
            v.push("-Werror".to_string());
        }
        if let Some(s) = &self.sanitize {
            v.push(s.clone());
        }
        v
    }
}

/// Map a sanitizer name to its clang flag.
fn sanitize_flag(name: &str) -> String {
    match name {
        "address" => "-fsanitize=address".to_string(),
        "thread" => "-fsanitize=thread".to_string(),
        "undefined" => "-fsanitize=undefined".to_string(),
        "memory" => "-fsanitize=memory".to_string(),
        other => other.to_string(),
    }
}

/// Probe for an executable at `<build>/<name>` or `<build>/bin`.
///
/// Binaries carry a `.exe` suffix on Windows, so we probe both the bare
/// name and the `.exe`-suffixed variant.
pub fn find_built_binary(project: &Project, name: &str) -> Option<PathBuf> {
    let build = project.build_dir();
    let exe_name = format!("{name}{}", std::env::consts::EXE_SUFFIX);
    let candidates = [
        build.join(&exe_name),
        build.join("bin"),
        build.join("bin").join(&exe_name),
        // Also probe the bare name for backward compatibility (Unix).
        build.join(name),
        build.join("bin").join(name),
    ];
    candidates.into_iter().find(|p| p.is_file())
}

/// The default (clang) backend: compile each source to an object file, then link.
fn clang_build(project: &Project, flags: &Flags, printer: &Printer) -> Result<()> {
    let build = project.build_dir();
    std::fs::create_dir_all(&build)?;
    let sources = project.sources()?;
    let mut objects = Vec::with_capacity(sources.len());

    // Compile each translation unit.
    for src in &sources {
        let obj = object_path(&build, &project.root, src);
        if let Some(parent) = obj.parent() {
            std::fs::create_dir_all(parent)?;
        }
        // clang++ (the C++ driver) links the C++ stdlib (libc++ on macOS,
        // libstdc++ on Linux) automatically; plain `clang` does not.
        let mut cmd = Command::new("clang++");
        cmd.args(flags.all());
        cmd.arg("-c").arg(src).arg("-o").arg(&obj);
        shell::run_step(
            &mut cmd,
            &format!("clang compile {}", pretty_rel(&project.root, src)),
            printer,
        )?;
        objects.push(obj);
    }

    // Link into <build>/<name> (with .exe suffix on Windows).
    let name = project_name(project);
    let bin = build.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
    let mut link = Command::new("clang++");
    link.args(flags.all()).args(&objects).arg("-o").arg(&bin);
    shell::run_step(
        &mut link,
        &format!("clang link {}", pretty_rel(&project.root, &bin)),
        printer,
    )?;

    // Emit a compile_commands.json so clangd works without cmake.
    write_compile_commands(project, &sources, flags)?;
    printer.info("compile_commands.json refreshed at project root");

    Ok(())
}

/// The cmake backend: (re)generate CMakeLists.txt from the template, configure,
/// build, then mirror compile_commands.json to the project root.
fn cmake_build(project: &Project, flags: &Flags, printer: &Printer) -> Result<()> {
    if !crate::toolchain::on_path("cmake") {
        return Err(CastleError::ToolMissing { tool: "cmake" });
    }

    // (Re)write CMakeLists.txt from the current manifest so flag changes are
    // picked up. We honor the manifest's standard but let the CLI override win.
    let cxx = flags
        .std_flag
        .strip_prefix("-std=c++")
        .unwrap_or("23")
        .to_string();
    let rendered = crate::templates::render(
        crate::templates::CMAKE_LISTS,
        &[
            ("PROJECT_NAME", &project_name(project)),
            ("CXX_STANDARD", &cxx),
        ],
    );
    std::fs::write(project.root.join("CMakeLists.txt"), rendered)?;

    let build = project.build_dir();
    std::fs::create_dir_all(&build)?;
    let mut configure = Command::new("cmake");
    configure
        .args(["-S", "."])
        .arg("-B")
        .arg(&build)
        // On Windows cmake defaults to the MSVC toolchain; force clang++
        // so the compiler matches what Castle's toolchain check expects.
        .arg("-DCMAKE_CXX_COMPILER=clang++");
    shell::run_step(&mut configure, "cmake configure", printer)?;

    let mut make = Command::new("cmake");
    make.arg("--build").arg(&build);
    shell::run_step(&mut make, "cmake build", printer)?;

    // Mirror compile_commands.json to the project root for clangd.
    let src_cc = build.join("compile_commands.json");
    let dst_cc = project.root.join("compile_commands.json");
    if src_cc.is_file() {
        let _ = std::fs::remove_file(&dst_cc);
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&src_cc, &dst_cc)
                .or_else(|_| std::fs::copy(&src_cc, &dst_cc).map(|_| ()))?;
        }
        #[cfg(not(unix))]
        {
            std::fs::copy(&src_cc, &dst_cc).map(|_| ())?;
        }
        printer.info("compile_commands.json mirrored to project root");
    }

    Ok(())
}

/// A single `compile_commands.json` entry. Paths are stored as strings
/// because `std::path::Path` does not implement `serde::Serialize`.
#[derive(Serialize)]
struct CompileEntry {
    directory: String,
    command: String,
    file: String,
}

/// Write a `compile_commands.json` describing the clang-backend compile commands.
fn write_compile_commands(project: &Project, sources: &[PathBuf], flags: &Flags) -> Result<()> {
    let root = &project.root;
    let flag_str = flags.all().join(" ");
    let entries: Vec<CompileEntry> = sources
        .iter()
        .map(|src| {
            let obj = object_path(&project.build_dir(), root, src);
            CompileEntry {
                directory: root.display().to_string(),
                command: format!(
                    "clang++ {flag_str} -c {} -o {}",
                    src.display(),
                    obj.display()
                ),
                file: src.display().to_string(),
            }
        })
        .collect();
    let json = serde_json::to_string_pretty(&entries)?;
    std::fs::write(root.join("compile_commands.json"), json + "\n")?;
    Ok(())
}

fn project_name(project: &Project) -> String {
    project
        .manifest
        .as_ref()
        .map(|m| m.package.name.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "bin".to_string())
}

fn object_path(build: &Path, root: &Path, src: &Path) -> PathBuf {
    let rel = src.strip_prefix(root).unwrap_or(src);
    let mut obj = build.join(rel);
    obj.set_extension("o");
    obj
}

fn pretty_rel(root: &Path, p: &Path) -> String {
    p.strip_prefix(root)
        .map(|r| r.display().to_string())
        .unwrap_or_else(|_| p.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named_manifest(name: &str) -> Manifest {
        Manifest {
            package: crate::manifest::Package {
                name: name.to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn flags_from_debug_profile() {
        let p = Profile::default();
        let f = Flags::from(&p, 23, None);
        assert_eq!(f.std_flag, "-std=c++23");
        assert_eq!(f.opt_flag, "-O0");
        assert!(f.debug);
        assert!(f.pedantic);
        assert!(f.sanitize.is_none());
        assert!(f.all().contains(&"-Wpedantic".to_string()));
    }

    #[test]
    fn flags_honor_sanitizer_override() {
        let p = Profile::default();
        let f = Flags::from(&p, 26, Some("address"));
        assert_eq!(f.std_flag, "-std=c++26");
        assert_eq!(f.sanitize.as_deref(), Some("-fsanitize=address"));
        assert!(f.all().iter().any(|s| s == "-fsanitize=address"));
    }

    #[test]
    fn flags_unknown_opt_level_collapses_to_o0() {
        let p = Profile {
            opt_level: "9".to_string(),
            ..Default::default()
        };
        let f = Flags::from(&p, 23, None);
        assert_eq!(f.opt_flag, "-O0");
    }

    #[test]
    fn backend_parse_known() {
        assert_eq!(Backend::parse("clang").unwrap(), Backend::Clang);
        assert_eq!(Backend::parse("cmake").unwrap(), Backend::Cmake);
        assert!(Backend::parse("ninja").is_err());
    }

    #[test]
    fn backend_resolve_cli_overrides_manifest() {
        let m = named_manifest("x");
        assert_eq!(
            Backend::resolve(Some(&m), Some("cmake")).unwrap(),
            Backend::Cmake
        );
    }

    #[test]
    fn backend_resolve_manifest_overrides_default() {
        let m = Manifest {
            backend: Some("cmake".to_string()),
            ..named_manifest("x")
        };
        assert_eq!(Backend::resolve(Some(&m), None).unwrap(), Backend::Cmake);
    }

    #[test]
    fn backend_resolve_defaults_to_clang() {
        assert_eq!(Backend::resolve(None, None).unwrap(), Backend::Clang);
    }
}
