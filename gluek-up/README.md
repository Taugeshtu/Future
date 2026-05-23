> "System-wide symbolic peek and definition jumping for spatial workspaces."

## why

`gluek-up` bridges this gap: it extracts the current cursor context from `Current`, queries the `lsp-broker` for definitions or references, and displays them in a centered, system-wide code browser overlay. Hitting `Enter` opens the target file precisely at the cursor location in a lightweight editor instance.

## how

`gluek-up` runs as a centered `gtk4-layer-shell` utility:
1. Connects to `current.sock` to query the focused file, line, and column.
2. Queries the `lsp-broker-query.sock` Unix socket for `textDocument/definition` or `textDocument/references`.
3. Displays matches in a dual-pane layout: a match list on the left, and a syntax-highlighted preview using `GtkSourceView` on the right.

### Keys

- `Up` / `Down` — Select match row (updates preview pane dynamically)
- `Return` — Open target in editor and exit
- `Esc` — Dismiss peek window

## Install

Dependencies: 
- [rust installed in your system](https://rust-lang.org/tools/install/)
- `GTK4`, `gtk4-layer-shell` system libraries. On Fedora:
```sh
sudo dnf install gtk4-devel gtk4-layer-shell-devel
```
_(have instructions for your repo? happy to add - make an issue with them!)_

Build and install:
```bash
# inside the gluek-up directory
cargo install --path . --root ~/.local
```
This puts `gluek-up` in `~/.local/bin/`.

### Configuration

`gluek-up` invokes the editor command specified in the `GLUEK_UP_EDITOR_CMD` environment variable, defaulting to `lite-xl` if unset.

To use:
```bash
export GLUEK_UP_EDITOR_CMD="lite-xl"
```
*(Ensure the custom Lite-XL `current.lua` plugin is loaded to enable the `file:line:col` coordinate-opening capability).*
