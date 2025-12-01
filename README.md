# RustOwl

_This fork adds support for the Helix editor and other editors that are able to read code actions from an LSP-server and simplifies the codebase considerably._

RustOwl visualizes ownership movement and lifetimes in Rust code. When you save a Rust file, RustOwl analyzes it and shows ownership/lifetime info when you hover over variables or function calls (or use a code action).

RustOwl uses colored underlines:

- 🟩 Green: variable's actual lifetime
- 🟦 Blue: immutable borrow
- 🟪 Purple: mutable borrow
- 🟧 Orange: value moved / function call
- 🟥 Red: lifetime error (invalid overlap or mismatch)

Move the cursor over a variable or function call and wait ~2 seconds to visualize info. RustOwl uses an extended LSP protocol, so it can be integrated with other editors.

## Installation

Prerequisites:

- `rustup` ([install](https://rustup.rs/))
- C compiler (`gcc`, `clang`, or Visual Studio on Windows)

RustOwl requires a nightly Rust toolchain, which will be installed automatically by `rustup` based on [rust-toolchain.toml](rust-toolchain.toml).

```bash
git clone git@github.com:wvhulle/rustowl.git /tmp/rustowl
cd /tmp/rustowl
cargo install --path . --locked
```

Then, complete the editor setup: see [editors/](./editors/)

## Usage

1. Open a Rust file in your editor (must be part of a Cargo workspace).
2. For VS Code, analysis starts automatically. For other editors, enable RustOwl manually or configure auto-loading.
3. Progress is shown in your editor. RustOwl works for analyzed portions, even if the whole workspace isn't finished.
4. Place the cursor on a variable or function call to inspect ownership/lifetime info.

## Notes

- VS Code: Underlines may not display perfectly for some characters (e.g., g, parentheses).
- `println!` macro may produce extra output (does not affect usability).
