use std::{
    collections::VecDeque,
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::{Command, exit},
};

use tokio::process::Command as TokioCommand;

/// Host target triple (set at compile time in build.rs)
pub const HOST_TUPLE: &str = env!("HOST_TUPLE");

/// Returns the Rust sysroot path for the compiler.
///
/// Resolution order:
/// 1. `RUSTOWL_SYSROOT` environment variable
/// 2. `rustc --print sysroot` output
pub fn get_sysroot() -> PathBuf {
    if let Ok(sysroot) = env::var("RUSTOWL_SYSROOT") {
        let path = PathBuf::from(sysroot);
        if path.is_dir() {
            log::info!("Using sysroot from RUSTOWL_SYSROOT: {}", path.display());
            return path;
        }
    }

    if let Ok(output) = Command::new("rustc").arg("--print").arg("sysroot").output()
        && output.status.success()
    {
        let sysroot = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let path = PathBuf::from(&sysroot);
        if path.is_dir() {
            log::info!("Using sysroot from rustc: {}", path.display());
            return path;
        }
    }

    log::error!(
        "Could not determine Rust sysroot. Set RUSTOWL_SYSROOT or ensure rustc is in PATH."
    );
    exit(1);
}

/// Returns the path to the current executable.
fn current_exe_path() -> PathBuf {
    env::current_exe().expect("Failed to get current executable path")
}

/// Creates a cargo command configured for `RustOwl` analysis.
///
/// Sets up environment variables so cargo uses the current rustowl binary
/// as the compiler wrapper.
pub fn setup_cargo_command() -> TokioCommand {
    let mut command = TokioCommand::new("cargo");
    let rustowl = current_exe_path();
    let sysroot = get_sysroot();

    command
        .env("RUSTC", &rustowl)
        .env("RUSTC_WORKSPACE_WRAPPER", &rustowl)
        .env("RUSTC_BOOTSTRAP", "1")
        .env(
            "CARGO_ENCODED_RUSTFLAGS",
            format!("--sysroot={}", sysroot.display()),
        );

    prepend_library_path(&mut command, &sysroot);
    command
}

fn prepend_library_path(command: &mut TokioCommand, sysroot: &Path) {
    let lib_dir = sysroot.join("lib");

    #[cfg(target_os = "linux")]
    {
        let paths = prepend_to_path_var("LD_LIBRARY_PATH", &lib_dir);
        command.env("LD_LIBRARY_PATH", paths);
    }

    #[cfg(target_os = "macos")]
    {
        let paths = prepend_to_path_var("DYLD_FALLBACK_LIBRARY_PATH", &lib_dir);
        command.env("DYLD_FALLBACK_LIBRARY_PATH", paths);
    }

    #[cfg(target_os = "windows")]
    {
        let paths = prepend_to_path_var("Path", &sysroot.join("bin"));
        command.env("Path", paths);
    }
}

fn prepend_to_path_var(var: &str, new_path: &Path) -> OsString {
    let current = env::var_os(var).unwrap_or_default();
    let mut paths: VecDeque<PathBuf> = env::split_paths(&current).collect();
    paths.push_front(new_path.to_path_buf());
    env::join_paths(paths).expect("Failed to join paths")
}
