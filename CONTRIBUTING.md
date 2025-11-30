# Contributing to RustOwl

Thank you for considering contributing to RustOwl!

## Development Setup

### Prerequisites

- Rust toolchain (automatically managed via `rust-toolchain.toml`)
- Basic build tools

For installation instructions, see [installation/](installation/).

For editor setup, see [editors/](editors/).

### Building

```bash
cargo build
cargo test
```

The project uses `rust-toolchain.toml` to automatically select the correct nightly toolchain.

## Before Submitting PR

### Rust Code

```bash
cargo fmt
cargo clippy --all-targets --all-features
cargo test
cargo build --release
```

### VS Code Extension

From `editors/vscode/`:

```bash
pnpm install
pnpm fmt
pnpm lint
pnpm check-types
pnpm test
```

### Neovim Plugin

From `editors/neovim/`:

```bash
./test.sh
```

Requires `nvim` in PATH.

### Security

```bash
cargo install cargo-deny
cargo deny check
```

Checks for:

- Security vulnerabilities (RustSec database)
- License compliance
- Duplicate dependencies

Configuration: [deny.toml](deny.toml)

### Optional: Memory Safety

```bash
cargo miri test
valgrind --leak-check=full ./target/release/rustowl  # Linux only
```
