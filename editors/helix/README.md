# RustOwl for Helix

This directory contains the Helix editor configuration for RustOwl LSP.

## Prerequisites

1. Install the `rustowl` binary (see [installation instructions](../../installation/README.md))
2. Ensure `rustowl` is in your `$PATH`

## Installation

Project-local:

```bash
cp . PROJECT/.helix
```

User-wide (for new Helix users)

```bash
cp . ~/.config/helix
```

## Usage

1. Open a Rust file in Helix and wait a bit
2. Simply press `<space>a` (code actions) while your cursor is on a variable, then select:

- **"RustOwl: Show ownership"** - Display ownership/lifetime diagnostics for the variable under cursor
- **"RustOwl: Hide ownership"** - Clear the diagnostics

That's it! The ownership information will appear as inline diagnostics.

## Troubleshooting

Ownership diagnostics take some time to be computed. In that case you will the text "analyzing..." and you need to wait a few seconds longer. Very large projects may require a longer time to get read.

If you still have problems, check Helix logs:

```bash
grep -i rustowl ~/.cache/helix/helix.log
```

## Limitations

Helix does not support custom LSP methods like `rustowl/cursor` that enable automatic hover-triggered decorations. The ownership visualization in Helix requires manual command invocation. For automatic hover-based visualization, consider using VS Code or Neovim with the dedicated plugins.
