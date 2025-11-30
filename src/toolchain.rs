use std::env;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

pub const TOOLCHAIN: &str = env!("RUSTOWL_TOOLCHAIN");
pub const HOST_TUPLE: &str = env!("HOST_TUPLE");

/// Returns a pre-configured sysroot from RUSTOWL_SYSROOT environment variable.
/// When set, this sysroot is used directly without toolchain downloading.
pub fn get_configured_sysroot() -> Option<PathBuf> {
    env::var("RUSTOWL_SYSROOT")
        .ok()
        .map(PathBuf::from)
        .filter(|p| {
            let exists = p.is_dir();
            if exists {
                log::info!(
                    "Using pre-configured sysroot from RUSTOWL_SYSROOT: {}",
                    p.display()
                );
            }
            exists
        })
}

/// Returns the runtime directory by checking standard locations.
/// Prefers RUSTOWL_RUNTIME_DIR, then derives from RUSTOWL_SYSROOT, then standard paths.
static RUNTIME_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    if let Ok(runtime_dir) = env::var("RUSTOWL_RUNTIME_DIR") {
        let path = PathBuf::from(&runtime_dir);
        log::info!("Using RUSTOWL_RUNTIME_DIR: {}", path.display());
        return path;
    }

    if let Some(parent) = get_configured_sysroot().and_then(|sysroot| {
        sysroot
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
    }) {
        log::info!(
            "Deriving runtime dir from RUSTOWL_SYSROOT: {}",
            parent.display()
        );
        return parent;
    }

    let opt = PathBuf::from("/opt/rustowl");
    if sysroot_from_runtime(&opt).is_dir() {
        return opt;
    }

    let same = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    if sysroot_from_runtime(&same).is_dir() {
        return same;
    }

    env::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".rustowl")
});

fn sysroot_from_runtime(runtime: impl AsRef<Path>) -> PathBuf {
    runtime.as_ref().join("sysroot").join(TOOLCHAIN)
}

pub fn get_sysroot() -> PathBuf {
    if let Some(sysroot) = get_configured_sysroot() {
        return sysroot;
    }

    let runtime = RUNTIME_DIR.clone();
    let sysroot = sysroot_from_runtime(&runtime);

    if !sysroot.is_dir() {
        log::error!(
            "Sysroot not found at {}. Please install the toolchain using the installation scripts in the installation/ directory.",
            sysroot.display()
        );
        std::process::exit(1);
    }

    sysroot
}

pub fn get_executable_path(name: &str) -> String {
    #[cfg(not(windows))]
    let exec_name = name.to_owned();
    #[cfg(windows)]
    let exec_name = format!("{name}.exe");

    let sysroot = get_sysroot();
    let exec_bin = sysroot.join("bin").join(&exec_name);
    if exec_bin.is_file() {
        log::info!("{name} is selected in sysroot/bin");
        return exec_bin.to_string_lossy().to_string();
    }

    if let Ok(current_exec) = env::current_exe() {
        let mut exec_path = current_exec;
        exec_path.set_file_name(&exec_name);
        if exec_path.is_file() {
            log::info!("{name} is selected in the same directory as rustowl executable");
            return exec_path.to_string_lossy().to_string();
        }
    }

    log::warn!("{name} not found; using fallback from PATH");
    exec_name
}

pub fn setup_cargo_command() -> tokio::process::Command {
    let cargo = get_executable_path("cargo");
    let mut command = tokio::process::Command::new(&cargo);
    let rustowlc = get_executable_path("rustowlc");
    command
        .env("RUSTC", &rustowlc)
        .env("RUSTC_WORKSPACE_WRAPPER", &rustowlc);
    set_rustc_env(&mut command, &get_sysroot());
    command
}

pub fn set_rustc_env(command: &mut tokio::process::Command, sysroot: &Path) {
    command.env("RUSTC_BOOTSTRAP", "1").env(
        "CARGO_ENCODED_RUSTFLAGS",
        format!("--sysroot={}", sysroot.display()),
    );

    #[cfg(target_os = "linux")]
    {
        let mut paths = env::split_paths(&env::var("LD_LIBRARY_PATH").unwrap_or_default())
            .collect::<std::collections::VecDeque<_>>();
        paths.push_front(sysroot.join("lib"));
        if let Ok(paths) = env::join_paths(paths) {
            command.env("LD_LIBRARY_PATH", paths);
        }
    }
    #[cfg(target_os = "macos")]
    {
        let mut paths =
            env::split_paths(&env::var("DYLD_FALLBACK_LIBRARY_PATH").unwrap_or_default())
                .collect::<std::collections::VecDeque<_>>();
        paths.push_front(sysroot.join("lib"));
        if let Ok(paths) = env::join_paths(paths) {
            command.env("DYLD_FALLBACK_LIBRARY_PATH", paths);
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Some(path_var) = env::var_os("Path") {
            let mut paths = env::split_paths(&path_var).collect::<std::collections::VecDeque<_>>();
            paths.push_front(sysroot.join("bin"));
            if let Ok(paths) = env::join_paths(paths) {
                command.env("Path", paths);
            }
        }
    }
}
