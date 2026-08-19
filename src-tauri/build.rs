#[path = "diagnostics.rs"]
mod diagnostics;

fn main() {
    // Execute diagnostic assertions and environment logging before build operations
    diagnostics::run_diagnostics();

    // Invoke Tauri build system
    tauri_build::build()
}
