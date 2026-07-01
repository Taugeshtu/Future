# Lifecycle

## `purse-niri` invocation path

```
Caller (script/action) invokes purse-niri
        │
        ▼
purse-niri queries Niri IPC
        │
        ├── purse on focused workspace → output existing PID to stdout → exit
        │
        └── no purse here → spawn `purse` → output new child PID to stdout → exit
```

`purse-niri` is always short-lived. It resolves, prints PID, and exits.

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
| Command / Script | `purse-niri` | invoked to find or spawn instance, returns PID |
| Niri IPC | `purse-niri` | workspace + window query |
| Unix socket (server) | `purse` | receives file paths/coordinates from any sender |
| Wayland DnD | `purse` | GTK4 DropTarget on the grid |
| Dispatch / launcher | `purse` | hands off selected file list (mechanism TBD) |
