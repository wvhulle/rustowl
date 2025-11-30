//! # RustOwl lib
//!
//! Libraries that used in RustOwl

pub mod cache;
pub mod cli;
pub mod lsp;
pub mod models;
pub mod shells;
pub mod toolchain;
pub mod utils;

pub use lsp::backend::Backend;

// Re-export for CLI usage
pub use crate::start_lsp_server_impl as start_lsp_server;

use tower_lsp::{LspService, Server};

/// Starts the LSP server
pub async fn start_lsp_server_impl() {
    use std::env;
    
    fn set_log_level(default: log::LevelFilter) {
        log::set_max_level(
            env::var("RUST_LOG")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(default),
        );
    }
    
    set_log_level("warn".parse().unwrap());
    eprintln!("RustOwl v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("This is an LSP server. You can use --help flag to show help.");

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(Backend::new)
        .custom_method("rustowl/cursor", Backend::cursor)
        .custom_method("rustowl/analyze", Backend::analyze)
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

// Miri-specific memory safety tests
#[cfg(test)]
mod miri_tests;
