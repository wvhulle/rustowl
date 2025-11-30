#![feature(rustc_private)]

extern crate indexmap;
extern crate polonius_engine;
extern crate rustc_borrowck;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_query_system;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_stable_hash;
extern crate rustc_type_ir;
extern crate smallvec;

mod cache;
pub mod cli;
pub mod compiler;
mod lsp;
mod models;
mod shells;
mod toolchain;
mod utils;

use lsp::backend::Backend;
use tokio::io;
use tower_lsp::{LspService, Server};

async fn start_lsp_server() {
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

    let stdin = io::stdin();
    let stdout = io::stdout();

    let (service, socket) = LspService::build(Backend::new)
        .custom_method("rustowl/cursor", Backend::cursor)
        .custom_method("rustowl/analyze", Backend::analyze)
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
