//! Typed errors for Castle.
//!
//! Every user-facing failure flows through [`CastleError`]. `Display` strings
//! are kept free of the `Castle:` prefix — the output layer adds it so messages
//! stay consistent whether printed directly or bubbled up via `?`.

use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// A `Result` specialised to Castle's error type.
pub type Result<T> = std::result::Result<T, CastleError>;

/// All the ways a Castle invocation can fail.
#[derive(Debug, Error)]
pub enum CastleError {
    /// `clang` was not found on `PATH`.
    #[error("clang compiler not found on PATH")]
    ClangNotFound,

    /// The located clang is older than the minimum required for the target standard.
    #[error("clang {found} is too old; Castle requires clang {required} or newer")]
    ClangTooOld { found: u32, required: u32 },

    /// `clang --version` produced output we couldn't parse.
    #[error("could not parse clang version from: {0}")]
    ClangVersionParse(String),

    /// Walked up the filesystem and found no Castle project marker.
    #[error("not inside a Castle project (no castle.toml or CMakeLists.txt found)")]
    NotInProject,

    /// `castle new`/`init` was pointed at an existing project.
    #[error("project already exists: {0}")]
    ProjectExists(PathBuf),

    /// A project name failed validation.
    #[error("invalid project name: {0}")]
    InvalidProjectName(String),

    /// No translation units were found where some were expected.
    #[error("no .cpp sources found under {dir}")]
    NoSources { dir: String },

    /// A required external tool is missing from `PATH`.
    #[error("required tool not found on PATH: {tool}")]
    ToolMissing { tool: &'static str },

    /// A build backend subprocess exited non-zero.
    #[error("{label} exited with status {code}")]
    BackendFailed { label: String, code: i32 },

    /// The manifest (`castle.toml`) could not be parsed.
    #[error("failed to parse castle.toml: {0}")]
    ManifestParse(String),

    /// An unknown value was supplied for a configuration option.
    #[error("invalid value for {field}: {value}")]
    InvalidValue { field: &'static str, value: String },

    /// `castle tidy` was run before any build produced `compile_commands.json`.
    #[error("no compile_commands.json found at project root")]
    NoCompileCommands,

    /// A filesystem watcher could not be established.
    #[error("file watcher error: {0}")]
    Watch(String),

    /// Failed to serialize `compile_commands.json`.
    #[error("failed to write compile_commands.json: {0}")]
    Json(#[from] serde_json::Error),

    /// A plain I/O error.
    #[error("{0}")]
    Io(#[from] io::Error),
}

impl From<notify::Error> for CastleError {
    fn from(e: notify::Error) -> Self {
        CastleError::Watch(e.to_string())
    }
}
