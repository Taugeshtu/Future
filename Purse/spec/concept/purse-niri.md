# `purse-niri` — Subsystems

## Niri IPC client

Connects to Niri's socket (`$NIRI_SOCKET` or default path) and queries:
- The currently focused workspace
- The list of windows on that workspace (app-id, PID, workspace membership)

## Instance resolution

Given the query results:

```
focused workspace known
    │
    ├── purse window exists on this workspace
    │       │
    │       └── locate its socket: $XDG_RUNTIME_DIR/purse-{pid}.sock
    │               │
    │               ├── socket reachable → forward files → exit
    │               └── socket not reachable → treat as absent (stale)
    │
    └── no purse window on this workspace
            │
            └── spawn `purse` with files as argv → exit
```

Stale socket (window listed by Niri but socket gone) is treated as absent: spawn fresh.

## File forwarding

Writes newline-delimited file paths to the target socket, then exits.
No persistent connection. Fire and forget.

## Invocation contract

Called by Thunar custom action (or anything else) with file paths as arguments:

```
purse-niri /path/to/file1 /path/to/file2 ...
```

Exits immediately after forwarding or spawning. Does not stay resident.
