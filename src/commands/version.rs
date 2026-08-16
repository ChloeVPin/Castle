//! `castle version` — print Castle and toolchain information.

use crate::error::Result;
use crate::output::Printer;
use crate::toolchain::detect_clang;

/// Print version + detected toolchain.
pub fn run(printer: &Printer) -> Result<()> {
    printer.info(&format!("castle {}", env!("CARGO_PKG_VERSION")));
    match detect_clang() {
        Ok(clang) => {
            let ver_line = clang
                .raw
                .lines()
                .next()
                .unwrap_or("clang (version unknown)");
            printer.plain(&format!("  clang       {ver_line}"));
            printer.plain("  C++ default c++23");
            for tool in ["clang++", "cmake", "ccache", "clang-format", "clang-tidy"] {
                if crate::toolchain::on_path(tool) {
                    printer.plain(&format!("  {tool:<11} available"));
                }
            }
        }
        Err(e) => {
            printer.warn(&format!("clang not detected: {e}"));
        }
    }
    Ok(())
}
