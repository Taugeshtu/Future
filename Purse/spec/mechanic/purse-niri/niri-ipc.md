# Niri IPC

`niri.rs` — connects to Niri's socket and queries workspace + window state.

## Socket

```rust
fn niri_socket_path() -> PathBuf {
    std::env::var("NIRI_SOCKET")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // fallback: niri default path
            let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or("/tmp".into());
            PathBuf::from(runtime).join("niri/socket")
        })
}
```

## Protocol

Niri speaks newline-terminated JSON over a Unix stream socket.
Write one request object, read one response object.

Using the `niri-ipc` crate (if available as a git dependency from the niri repo)
gives typed request/response enums. If not practical to depend on, implement
the two queries we need as raw serde_json:

### Query: focused workspace

Request:
```json
{"Workspaces": null}
```
Response: array of workspace objects. The focused one has `"is_focused": true`.
We need its `"id"` field (a `u64`).

### Query: window list

Request:
```json
{"Windows": null}
```
Response: array of window objects. Fields we use:
- `"app_id": Option<String>` — we match on `"dev.purse"`
- `"workspace_id": Option<u64>` — cross-reference with focused workspace id
- `"title": Option<String>` — contains `"purse-{pid}"` per entry-point convention

## Structs (if implementing without niri-ipc crate)

```rust
#[derive(Deserialize)]
struct NiriWorkspace {
    id: u64,
    is_focused: bool,
}

#[derive(Deserialize)]
struct NiriWindow {
    app_id: Option<String>,
    workspace_id: Option<u64>,
    title: Option<String>,
}
```

## PID extraction from title

```rust
fn pid_from_title(title: &str) -> Option<u32> {
    title.strip_prefix("purse-")?.parse().ok()
}
```

This is the coupling point between `purse`'s title convention and `purse-niri`.
If the title format changes, update both sides.

## Public interface

```rust
pub struct NiriState {
    pub focused_workspace_id: Option<u64>,
    pub purse_windows: Vec<PurseWindow>,
}

pub struct PurseWindow {
    pub workspace_id: u64,
    pub pid: u32,
}

pub fn query() -> anyhow::Result<NiriState> { ... }
```

`query()` makes both requests in sequence on the same connection and returns
the aggregated result. Errors (socket not found, parse failure) propagate to
the caller in `resolution.rs`.
