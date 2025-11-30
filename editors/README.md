### VS Code

You can install VS Code extension from [this link](https://marketplace.visualstudio.com/items?itemName=cordx56.rustowl-vscode).
RustOwl will be installed automatically when the extension is activated.

### Vscodium

You can install Vscodium extension from [this link](https://open-vsx.org/extension/cordx56/rustowl-vscode).
RustOwl will be installed automatically when the extension is activated.

After installation, the extension will automatically run RustOwl when you save any Rust program in cargo workspace.
The initial analysis may take some time, but from the second run onward, compile caching is used to reduce the analysis time.

## Other editor support

We support Neovim and Emacs.
You have to [install RustOwl](docs/installation.md) before using RustOwl with other editors.

You can also create your own LSP client.
If you would like to implement a client, please refer to the [The RustOwl LSP specification](docs/lsp-spec.md).

### Neovim

Minimal setup with [lazy.nvim](https://github.com/folke/lazy.nvim):

```lua
{
  'cordx56/rustowl',
  version = '*', -- Latest stable version
  build = 'cargo binstall rustowl',
  lazy = false, -- This plugin is already lazy
  opts = {},
}
```

For comprehensive configuration options including custom highlight colors, see the [Neovim Configuration Guide](docs/neovim-configuration.md).

<details>
<summary>Recommended configuration: <b>Click to expand</b></summary>

```lua
{
  'cordx56/rustowl',
  version = '*', -- Latest stable version
  build = 'cargo binstall rustowl',
  lazy = false, -- This plugin is already lazy
  opts = {
    client = {
      on_attach = function(_, buffer)
        vim.keymap.set('n', '<leader>o', function()
          require('rustowl').toggle(buffer)
        end, { buffer = buffer, desc = 'Toggle RustOwl' })
      end
    },
  },
}
```

</details>

Default options:

```lua
{
  auto_attach = true, -- Auto attach the RustOwl LSP client when opening a Rust file
  auto_enable = false, -- Enable RustOwl immediately when attaching the LSP client
  idle_time = 500, -- Time in milliseconds to hover with the cursor before triggering RustOwl
  client = {}, -- LSP client configuration that gets passed to `vim.lsp.start`
  highlight_style = 'undercurl', -- You can also use 'underline'
  colors = { -- Customize highlight colors (hex colors)
    lifetime = '#00cc00',   -- 🟩 green: variable's actual lifetime
    imm_borrow = '#0000cc', -- 🟦 blue: immutable borrowing
    mut_borrow = '#cc00cc', -- 🟪 purple: mutable borrowing
    move = '#cccc00',       -- 🟧 orange: value moved
    call = '#cccc00',       -- 🟧 orange: function call
    outlive = '#cc0000',    -- 🟥 red: lifetime error
  },
}
```

When opening a Rust file, the Neovim plugin creates the `Rustowl` user command:

```vim
:Rustowl {subcommand}
```

where `{subcommand}` can be one of:

- `start_client`: Start the rustowl LSP client.
- `stop_client`: Stop the rustowl LSP client.
- `restart_client`: Restart the rustowl LSP client.
- `enable`: Enable rustowl highlights.
- `disable`: Disable rustowl highlights.
- `toggle`: Toggle rustowl highlights.

### Emacs

Elpaca example:

```elisp
(elpaca
  (rustowl
    :host github
    :repo "cordx56/rustowl"))
```

Then use-package:

```elisp
(use-package rustowl
  :after lsp-mode)
```

You have to install RustOwl LSP server manually.

### RustRover / IntelliJ IDEs

There is a [third-party repository](https://github.com/siketyan/intellij-rustowl) that supports IntelliJ IDEs.
You have to install RustOwl LSP server manually.

### Sublime Text

There is a [third-party repository](https://github.com/CREAsTIVE/LSP-rustowl) that supports Sublime Text.
