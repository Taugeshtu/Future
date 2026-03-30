# Instance Resolution

`resolution.rs` — decides whether to forward to an existing Purse or spawn a new one.

## Decision tree

```rust
pub enum Resolution {
    Forward { pid: u32 },
    Spawn,
}

pub fn resolve(niri: &NiriState) -> Resolution {
    let focused_ws = match niri.focused_workspace_id {
        Some(id) => id,
        None => return Resolution::Spawn,   // can't determine workspace; spawn fresh
    };

    // find a purse window on the focused workspace
    let candidate = niri.purse_windows.iter()
        .find(|w| w.workspace_id == focused_ws);

    match candidate {
        None => Resolution::Spawn,
        Some(w) => {
            // verify the socket actually exists and is reachable
            if socket_is_live(w.pid) {
                Resolution::Forward { pid: w.pid }
            } else {
                Resolution::Spawn   // stale entry; niri knows the window but socket is gone
            }
        }
    }
}

fn socket_is_live(pid: u32) -> bool {
    let path = purse_socket_path(pid);
    // attempt a connection; if it succeeds the socket is live
    std::os::unix::net::UnixStream::connect(&path).is_ok()
}
```

`purse_socket_path(pid)` mirrors the path formula from `purse/ipc.rs`:
`$XDG_RUNTIME_DIR/purse-{pid}.sock`. Keep this formula in one shared place
or duplicate it carefully — it must match exactly.

## Edge cases

**Niri socket not available** (`$NIRI_SOCKET` not set, socket file missing):
`niri::query()` returns `Err`. Fall through to `Resolution::Spawn`.
This means `purse-niri` degrades gracefully outside Niri: it always spawns a new
Purse without workspace awareness. Fine.

**Multiple Purse windows on the same workspace** (violates the one-per-workspace
convention, but could happen): `find()` returns the first match. Which one is
first depends on Niri's window list order. This is undefined behaviour for now —
the convention is one per workspace; enforce it via usage, not code.

**Purse window on workspace but process died, socket stale**: `socket_is_live`
returns false → `Spawn`. The dead window lingers in Niri's list until Niri notices
the process is gone (typically immediate). Transient race; harmless.
