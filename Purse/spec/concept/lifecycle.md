# Lifecycle

## `purse-niri` invocation path

```
Thunar custom action fires with file list
        │
        ▼
purse-niri queries Niri IPC
        │
        ├── purse on this workspace → send files to socket → exit
        │
        └── no purse here → spawn `purse [files...]` → exit
```

`purse-niri` is always short-lived. It resolves and exits.

## `purse` window lifetime

```
spawned with initial file list (argv)
        │
        ▼
bind socket at $XDG_RUNTIME_DIR/purse-{pid}.sock
        │
        ▼
ingest initial files → render grid → window appears (floating)
        │
        ├── [IPC] more files arrive → ingest → re-render
        ├── [DnD] files dropped onto window → ingest → re-render
        ├── [UI]  user clicks items → toggle selection
        │
        ▼
user acts on selection
        ├── Enter → dispatch selected files to viewer(s)
        └── Ctrl+C → copy focused item content to clipboard
        │
        ▼
window closed
        │
        ▼
release socket → exit
```

## Integration points

| Point | Who | What |
|-------|-----|-------|
| Thunar custom action | `purse-niri` | invoked with file paths as argv |
| Niri IPC | `purse-niri` | workspace + window query |
| Unix socket (server) | `purse` | receives file paths from any sender |
| Unix socket (client) | `purse-niri` | forwards file paths to existing instance |
| Wayland DnD | `purse` | GTK4 DropTarget on the grid |
| Dispatch / launcher | `purse` | hands off selected file list (mechanism TBD) |
