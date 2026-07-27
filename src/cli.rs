//! Command-line interface definition (clap derive).
//!
//! Every subcommand maps to a handler in [`crate::commands`]. Help text is
//! written to match the friendly `Castle:` voice used everywhere else.

use clap::{Parser, Subcommand};

/// 🏰 Castle — a minimal Cargo-inspired C++ project manager.
///
/// Castle scaffolds, builds, and runs C++ projects with clang (≥ 18) and
/// C++23 by default. It is intentionally small: think "Cargo's grandchild,
/// for C++, on clang."
#[derive(Parser, Debug)]
#[command(name = "castle", version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a new Castle project in a new directory.
    New {
        /// Project (and output binary) name. Must be ASCII, no spaces.
        name: String,
        /// Generate a CMake-based project instead of the default clang-direct one.
        #[arg(long)]
        cmake: bool,
    },
    /// Create a new Castle project in the current directory.
    Init {
        /// Project (and output binary) name. Defaults to the current directory name.
        name: Option<String>,
        /// Generate a CMake-based project instead of the default clang-direct one.
        #[arg(long)]
        cmake: bool,
    },
    /// Build the project.
    Build {
        /// Build in release mode (opt-level 2, no debug info).
        #[arg(long)]
        release: bool,
        /// C++ standard to target (e.g. 23, 26).
        #[arg(long, value_name = "NN")]
        cxx_standard: Option<u32>,
        /// Build backend override: "clang" (default) or "cmake".
        #[arg(long, value_parser = ["clang", "cmake"])]
        backend: Option<String>,
        /// Enable AddressSanitizer.
        #[arg(long)]
        asan: bool,
        /// Enable ThreadSanitizer.
        #[arg(long)]
        tsan: bool,
        /// Enable UndefinedBehaviorSanitizer.
        #[arg(long)]
        ubsan: bool,
        /// Enable MemorySanitizer.
        #[arg(long)]
        msan: bool,
        /// Treat warnings as errors.
        #[arg(long)]
        pedantic: bool,
    },
    /// Build and run the project.
    Run {
        /// Build in release mode (opt-level 2, no debug info).
        #[arg(long)]
        release: bool,
        /// C++ standard to target (e.g. 23, 26).
        #[arg(long, value_name = "NN")]
        cxx_standard: Option<u32>,
        /// Build backend override.
        #[arg(long, value_parser = ["clang", "cmake"])]
        backend: Option<String>,
        /// Enable AddressSanitizer.
        #[arg(long)]
        asan: bool,
        /// Enable ThreadSanitizer.
        #[arg(long)]
        tsan: bool,
        /// Enable UndefinedBehaviorSanitizer.
        #[arg(long)]
        ubsan: bool,
        /// Enable MemorySanitizer.
        #[arg(long)]
        msan: bool,
        /// Watch for source changes and rebuild/re-run.
        #[arg(long)]
        watch: bool,
        /// Arguments to pass through to the program (everything after `--`).
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// Run a fast type/syntax check without linking.
    Check {
        /// C++ standard to target.
        #[arg(long, value_name = "NN")]
        cxx_standard: Option<u32>,
    },
    /// Remove build artifacts.
    Clean {
        /// Skip the confirmation prompt.
        #[arg(long, short = 'f')]
        force: bool,
    },
    /// Build and run the project's tests (the `tests/` directory).
    Test {
        /// C++ standard to target.
        #[arg(long, value_name = "NN")]
        cxx_standard: Option<u32>,
    },
    /// Run clang-format on the project's sources.
    Fmt,
    /// Run clang-tidy on the project's sources (requires a prior `castle build`).
    Tidy,
    /// Print version and detected toolchain information.
    Version,
}

impl Command {
    /// Return any sanitizer CLI flag that was set, in priority order.
    #[must_use]
    pub fn sanitizer_override(&self) -> Option<&'static str> {
        match self {
            Self::Build {
                asan,
                tsan,
                ubsan,
                msan,
                ..
            }
            | Self::Run {
                asan,
                tsan,
                ubsan,
                msan,
                ..
            } => {
                if *asan {
                    Some("address")
                } else if *tsan {
                    Some("thread")
                } else if *ubsan {
                    Some("undefined")
                } else if *msan {
                    Some("memory")
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
