//! Subcommand handlers. One module per command for easy navigation.

pub mod build;
pub mod check;
pub mod clean;
pub mod common;
pub mod fmt;
pub mod init;
pub mod new;
pub mod run;
pub mod test;
pub mod tidy;
pub mod version;

pub use build::run as run_build;
pub use check::run as run_check;
pub use clean::run as run_clean;
pub use fmt::run as run_fmt;
pub use init::run as run_init;
pub use new::run as run_new;
pub use run::run as run_run;
pub use test::run as run_test;
pub use tidy::run as run_tidy;
pub use version::run as run_version;
