# `purse-niri` — Subsystems

## Niri IPC client

Connects to Niri's socket (`$NIRI_SOCKET` or default path) and queries:
- The currently focused workspace
- The list of windows on that workspace (app-id, PID, workspace membership)

## Instance resolution and PID output

Given the query results:

```
focused workspace known
    │
    ├── purse window exists on this workspace
    │       │
    │       └── locate its socket: $XDG_RUNTIME_DIR/purse-{pid}.sock
    │               │
    │               ├── socket reachable → output PID to stdout → exit
    │               └── socket not reachable → treat as absent (stale)
    │
    └── no purse window on this workspace
            │
            └── spawn `purse` (no arguments) → output child PID to stdout → exit
```

Stale socket (window listed by Niri but socket gone) is treated as absent: spawn fresh.

## Invocation contract

Called without arguments. Outputs the target Purse's PID to stdout:

```
$ purse-niri
41235
```

Caller is responsible for connecting to `$XDG_RUNTIME_DIR/purse-{pid}.sock` and writing file paths.
