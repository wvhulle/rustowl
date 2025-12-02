use std::{env, process::Command};

fn main() {
    // Set host tuple for runtime target detection
    let host_tuple = get_host_tuple();
    println!("cargo::rustc-env=HOST_TUPLE={host_tuple}");

    // Set compile-time sysroot for finding rustc libraries
    let sysroot = get_sysroot();
    println!("cargo::rustc-env=COMPILE_TIME_SYSROOT={sysroot}");

    // Set rpath for dynamic linking to rustc libraries
    #[cfg(target_os = "macos")]
    println!("cargo::rustc-link-arg=-Wl,-rpath,@executable_path/../lib");

    #[cfg(target_os = "linux")]
    println!("cargo::rustc-link-arg=-Wl,-rpath,$ORIGIN/../lib");

    #[cfg(target_os = "windows")]
    println!("cargo::rustc-link-arg=/LIBPATH:..\\bin");
}

fn get_host_tuple() -> String {
    Command::new(env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string()))
        .arg("--print")
        .arg("host-tuple")
        .output()
        .map(|v| String::from_utf8(v.stdout).unwrap().trim().to_string())
        .expect("failed to obtain host-tuple")
}

fn get_sysroot() -> String {
    Command::new(env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string()))
        .arg("--print")
        .arg("sysroot")
        .output()
        .map(|v| String::from_utf8(v.stdout).unwrap().trim().to_string())
        .expect("failed to obtain sysroot")
}
