//! End-to-end round-trip test.
//!
//! Scaffolds a real Castle project in a tempdir using the `castle` binary,
//! then exercises `build` → `run` → `check` → `clean` against it. This is the
//! closest analog to the manual verification in the README.

use std::path::PathBuf;
use std::process::Command;

/// Path to the freshly built `castle` binary (set by `cargo test` via the
/// `CARGO_BIN_EXE_castle` env var).
fn castle_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_castle"))
}

/// Run `castle <args>` with `cwd` as the working directory.
fn castle_in(cwd: &std::path::Path, args: &[&str]) -> (bool, String) {
    let out = Command::new(castle_bin())
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("failed to spawn castle");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let combined = if stdout.is_empty() {
        stderr
    } else if stderr.is_empty() {
        stdout
    } else {
        format!("{stdout}\n--- stderr ---\n{stderr}")
    };
    (out.status.success(), combined)
}

#[test]
fn new_build_run_check_clean_round_trip() {
    // `castle new` needs clang ≥ 18 to build; skip gracefully if absent.
    let clang_ok = Command::new("clang")
        .arg("--version")
        .output()
        .is_ok_and(|o| o.status.success());
    if !clang_ok {
        eprintln!("skipping round-trip: clang not available");
        return;
    }

    let parent = tempfile::tempdir().expect("tempdir");
    let project_dir = parent.path().join("demo");

    // 1. `castle new demo`
    let (ok, out) = castle_in(parent.path(), &["new", "demo"]);
    assert!(ok, "`castle new demo` failed:\n{out}");
    assert!(project_dir.is_dir(), "project dir was not created");
    assert!(
        project_dir.join("castle.toml").is_file(),
        "castle.toml missing"
    );
    // Verify the {{PROJECT_NAME}} template rendering actually worked.
    let toml = std::fs::read_to_string(project_dir.join("castle.toml")).unwrap();
    assert!(
        toml.contains("demo"),
        "castle.toml did not contain project name 'demo':\n{toml}"
    );
    assert!(
        project_dir.join("src/main.cpp").is_file(),
        "src/main.cpp missing"
    );
    assert!(
        project_dir.join("tests/main.cpp").is_file(),
        "tests/main.cpp missing"
    );
    assert!(
        project_dir.join(".clang-format").is_file(),
        ".clang-format missing"
    );
    assert!(
        project_dir.join(".clang-tidy").is_file(),
        ".clang-tidy missing"
    );

    // 2. `castle build` (default clang backend; needs no cmake)
    let (ok, out) = castle_in(&project_dir, &["build"]);
    assert!(ok, "`castle build` failed:\n{out}");
    // The default clang backend writes a compile_commands.json at the root.
    assert!(
        project_dir.join("compile_commands.json").is_file(),
        "compile_commands.json not written by build"
    );

    // 3. `castle run` — the scaffolded main prints "Hello, world!"
    let (ok, out) = castle_in(&project_dir, &["run"]);
    assert!(ok, "`castle run` failed:\n{out}");
    assert!(
        out.contains("Hello, world!"),
        "program output did not contain hello-world:\n{out}"
    );

    // 4. `castle check` — syntax-only pass over the sources
    let (ok, out) = castle_in(&project_dir, &["check"]);
    assert!(ok, "`castle check` failed:\n{out}");

    // 4b. `castle test` — build and run the scaffolded tests/main.cpp
    let (ok, out) = castle_in(&project_dir, &["test"]);
    assert!(ok, "`castle test` failed:\n{out}");
    assert!(out.contains("ok"), "test output did not report ok:\n{out}");

    // 5. `castle clean --force` — removes build/ and compile_commands.json
    let (ok, out) = castle_in(&project_dir, &["clean", "--force"]);
    assert!(ok, "`castle clean --force` failed:\n{out}");
    assert!(
        !project_dir.join("build").exists(),
        "build/ still present after clean"
    );
    assert!(
        !project_dir.join("compile_commands.json").exists(),
        "compile_commands.json still present after clean"
    );
}

#[test]
fn version_reports_castle_and_clang() {
    let (ok, out) = castle_in(std::env::temp_dir().as_ref(), &["version"]);
    assert!(ok, "`castle version` failed:\n{out}");
    assert!(
        out.contains("castle"),
        "version output missing 'castle':\n{out}"
    );
}

#[test]
fn help_lists_subcommands() {
    let (ok, out) = castle_in(std::env::temp_dir().as_ref(), &["--help"]);
    assert!(ok, "`castle --help` failed:\n{out}");
    for sub in [
        "new", "init", "build", "run", "check", "clean", "test", "fmt", "tidy", "version",
    ] {
        assert!(
            out.contains(sub),
            "`castle --help` did not mention `{sub}`:\n{out}"
        );
    }
}

#[test]
fn new_rejects_existing_project() {
    let parent = tempfile::tempdir().expect("tempdir");
    let project_dir = parent.path().join("dup");
    std::fs::create_dir_all(&project_dir).unwrap();

    let (ok, out) = castle_in(parent.path(), &["new", "dup"]);
    assert!(
        !ok,
        "`castle new dup` should have failed on existing dir:\n{out}"
    );
    assert!(
        out.contains("already exists"),
        "expected 'already exists' message:\n{out}"
    );
}
