# FerrousOwl

FerrousOwl visualizes ownership movement and lifetimes in Rust code using colored underlines:

- 🟩 Green: variable's actual lifetime
- 🟦 Blue: immutable borrow
- 🟪 Purple: mutable borrow
- 🟧 Orange: value moved / function call
- 🟥 Red: lifetime error (invalid overlap or mismatch)

## Usage

It should be straight-forward to use this extension:

1. Open a Rust file
2. Click on a variable in your code.
3. Open the command panel of VS Code (with CTRL+SHIFT+P).
4. Type `FerrousOwl: Toggle` and press enter.

This will show the movement of ownership of the focused variable through the neighbouring code.

Move your mouse over the neighbouring underlined code to see what their relation is with the variable focused in the editor.

You can also enable automatic highlighting using the command `FerrousOwl: Cycle`.

## Installation

This extension should activate upon opening a Rust file. The system binary `ferrous-owl` should normally be installed automatically when the extension is activated. If not, you can install it manually, see the [FerrousOwl Rust binary](https://github.com/wvhulle/ferrous-owl). If that fails as well, please create a bug report.

## Note

Underlines may not display perfectly for some characters (e.g., g, parentheses).

Thanks a lot to the original author Yuki Okamoto!

_This fork of [RustOwl](https://github.com/cordx56/rustowl) adds support for the Helix editor and other editors that are able to read code actions from an LSP-server and simplifies the codebase considerably._
