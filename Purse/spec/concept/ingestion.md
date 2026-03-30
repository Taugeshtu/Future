# Ingestion

Ingestion is the process of turning a file path into a Purse item.

## Sources

All three sources are equivalent after ingestion. Same pipeline, same result.

| Source | Mechanism |
|--------|-----------|
| Initial load | CLI argv: `purse /path/a /path/b ...` |
| Live addition | IPC socket: newline-delimited paths written by `purse-niri` or any sender |
| Drag and drop | GTK4 `DropTarget` on the grid window, yields `gdk::FileList` |

## What ingestion does

1. Resolve path to canonical form
2. Check against current state: already held → skip (see state.md)
3. Determine MIME type
4. Kick off preview generation (async, best-effort)
5. Add item to state → triggers render update

## A Purse item

The unit Purse holds:

- Canonical file path
- MIME type (determined at ingestion)
- Preview payload (populated async after ingestion; may be pending/failed)
- Selection state (starts: selected)
