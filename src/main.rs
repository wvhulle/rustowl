//! # RustOwl cargo-owlsp
//!
//! An LSP server for visualizing ownership and lifetimes in Rust, designed for debugging and optimization.


use clap::Parser;
use rustowl::cli::Cli;
use std::env;

#[cfg(all(not(target_env = "msvc"), not(miri)))]
use tikv_jemallocator::Jemalloc;

#[cfg(all(not(target_env = "msvc"), not(miri)))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn set_log_level(default: log::LevelFilter) {
    log::set_max_level(
        env::var("RUST_LOG")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default),
    );
}

fn initialize_logging() {
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .init()
        .unwrap();
    set_log_level("info".parse().unwrap());
}

#[tokio::main]
async fn main() {
    initialize_logging();
    Cli::parse().run().await;
}
