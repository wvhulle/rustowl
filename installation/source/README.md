## Building from source

Prerequisites:

- `rustup` ([install](https://rustup.rs/))
- C compiler (`gcc`, `clang`, or Visual Studio on Windows)

RustOwl requires a nightly Rust toolchain, which will be installed automatically by `rustup` based on [rust-toolchain.toml](../../rust-toolchain.toml).

### Manual installation

```bash
cargo install --path . --locked
```

### Using install script

```bash
./installation/source/install.sh
```

The script accepts environment variables:

- `INSTALL_DIR`: Installation directory (default: `$HOME/.cargo/bin`)
- `BUILD_PROFILE`: `release` or `debug` (default: `release`)

### Runtime configuration

Customize runtime directory paths using environment variables:

- `RUSTOWL_RUNTIME_DIRS` or `RUSTOWL_SYSROOTS` (default: `$HOME/.rustowl`)

### Platform-specific notes

On Ubuntu systems, install build tools first:

```bash
apt install build-essential
```
