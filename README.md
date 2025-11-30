<div align="center">
    <h1>
      <picture>
        <source media="(prefers-color-scheme: dark)" srcset="docs/assets/rustowl-logo-dark.svg">
        <img alt="RustOwl" src="docs/assets/rustowl-logo.svg" width="400">
      </picture>
    </h1>
    <p>
        Visualize ownership and lifetimes in Rust for debugging and optimization
    </p>
    <h4>
        <a href="https://crates.io/crates/rustowl">
            <img alt="Crates.io Version" src="https://img.shields.io/crates/v/rustowl?style=for-the-badge">
        </a>
        <a href="https://aur.archlinux.org/packages/rustowl-bin">
            <img alt="AUR Version" src="https://img.shields.io/aur/version/rustowl-bin?style=for-the-badge">
        </a>
        <img alt="WinGet Package Version" src="https://img.shields.io/winget/v/Cordx56.Rustowl?style=for-the-badge">
    </h4>
    <h4>
        <a href="https://marketplace.visualstudio.com/items?itemName=cordx56.rustowl-vscode">
            <img alt="Visual Studio Marketplace Version" src="https://img.shields.io/visual-studio-marketplace/v/cordx56.rustowl-vscode?style=for-the-badge&label=VS%20Code">
        </a>
        <a href="https://open-vsx.org/extension/cordx56/rustowl-vscode">
            <img alt="Open VSX Version" src="https://img.shields.io/open-vsx/v/cordx56/rustowl-vscode?style=for-the-badge">
        </a>
        <a href="https://github.com/siketyan/intellij-rustowl">
            <img alt="JetBrains Plugin Version" src="https://img.shields.io/jetbrains/plugin/v/26504-rustowl?style=for-the-badge">
        </a>
    </h4>
    <p>
        <img src="docs/assets/readme-screenshot-3.png" />
    </p>
</div>

RustOwl visualizes ownership movement and lifetimes of variables.
When you save Rust source code, it is analyzed, and the ownership and lifetimes of variables are visualized when you hover over a variable or function call.

RustOwl visualizes those by using underlines:

- 🟩 green: variable's actual lifetime
- 🟦 blue: immutable borrowing
- 🟪 purple: mutable borrowing
- 🟧 orange: value moved / function call
- 🟥 red: lifetime error
  - diff of lifetime between actual and expected, or
  - invalid overlapped lifetime of mutable and shared (immutable) references

Detailed usage is described [here](docs/usage.md).

Currently, we offer VSCode extension, Neovim plugin and Emacs package.
For these editors, move the text cursor over the variable or function call you want to inspect and wait for 2 seconds to visualize the information.
We implemented LSP server with an extended protocol.
So, RustOwl can be used easily from other editor.

In this tool, due to the limitations of VS Code's decoration specifications, characters with descenders, such as g or parentheses, may occasionally not display underlines properly.
Additionally, we observed that the `println!` macro sometimes produces extra output, though this does not affect usability in any significant way.

## Installation

For package manager installations, see [installation/README.md](installation/README.md).

For building from source, see [installation/source/README.md](installation/source/README.md).
