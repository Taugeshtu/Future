# Glue Scripts

These scripts integrate the workspace context and language server facilities with **Purse** for spatial code peeking and navigation.

## Binaries / Scripts

- `purse-defs.sh`: Fetches definitions for the symbol under the cursor from `lsp-broker` and stages them in the current active Purse.
- `purse-refs.sh`: Fetches references for the symbol under the cursor from `lsp-broker` and stages them in the current active Purse.

## How it works

1. Queries `/run/user/1000/current.sock` to get the focused file, line, and column.
2. Queries `/run/user/1000/lsp-broker-query.sock` to resolve LSP definitions/references.
3. Obtains target locations, decodes URIs, and translates them to `file:line:col` coordinates.
4. Invokes `purse-niri` to locate (or spawn) the active workspace's Purse window and get its PID.
5. Sends coordinates directly to `$XDG_RUNTIME_DIR/purse-${PID}.sock` Unix socket using Netcat.
