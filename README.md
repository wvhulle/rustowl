# RustOwl

_This fork adds support for the Helix editor and other editors that are able to read code actions from an LSP-server and simplifies the codebase considerably._

RustOwl visualizes ownership movement and lifetimes in Rust code. When you save a Rust file, RustOwl analyzes it and shows ownership/lifetime info when you hover over variables or function calls.

## How It Works

RustOwl uses colored underlines:

- 🟩 Green: variable's actual lifetime
- 🟦 Blue: immutable borrow
- 🟪 Purple: mutable borrow
- 🟧 Orange: value moved / function call
- 🟥 Red: lifetime error (invalid overlap or mismatch)

Move the cursor over a variable or function call and wait ~2 seconds to visualize info. RustOwl uses an extended LSP protocol, so it can be integrated with other editors.

## Installation

First install the `rustowl` binary:

- Package managers: see [installation/README.md](installation/README.md)
- Build from source: see [installation/source/README.md](installation/source/README.md)

Then, complete the editor setup: see [editors/](./editors/)

## Getting Started

1. Open a Rust file in your editor (must be part of a Cargo workspace).
2. For VS Code, analysis starts automatically. For other editors, enable RustOwl manually or configure auto-loading.
3. Progress is shown in your editor. RustOwl works for analyzed portions, even if the whole workspace isn't finished.
4. Place the cursor on a variable or function call to inspect ownership/lifetime info.

## Usage

RustOwl helps resolve ownership and lifetime errors. It visualizes:

- Actual lifetime of variables
- Shared (immutable) borrowing
- Mutable borrowing
- Value movement
- Function calls
- Ownership/lifetime errors

Hover over underlined code for explanations (VS Code example below):

![Hover message on VS Code](assets/vscode-screenshot.png)

## Advanced Usage

RustOwl visualizes:

- Where a variable lives (_NLL_)
- Until it's dropped or moved (_RAII_)

Use RustOwl to:

- Resolve deadlocks (e.g., with `Mutex`)
- Manage resources (memory, files, etc. via RAII)

## Notes

- VS Code: Underlines may not display perfectly for some characters (e.g., g, parentheses).
- `println!` macro may produce extra output (does not affect usability).

---

For more details, see the documentation in [docs/](docs/).
