# Ownership and Threading

## State ownership

`PurseState` is wrapped in `Rc<RefCell<PurseState>>` and cloned (refcount bump)
into every GTK closure that needs it. `Rc` is correct here: GTK callbacks run
on the main thread only; no `Send` requirement, no `Arc` overhead needed.

```rust
let state = Rc::new(RefCell::new(PurseState { items: vec![], hover: None }));
```

Closures capture `state.clone()` and call `state.borrow()` / `state.borrow_mut()`
as needed. Panic on double-borrow is acceptable for v1 — do not hold a borrow
across any await point or nested callback.

## Preview channel

Preview generation runs in OS threads (blocking file I/O, image decode, poppler).
Results return to the main thread via a `glib::MainContext::channel`:

```rust
// in main.rs setup:
let (preview_tx, preview_rx) = glib::MainContext::channel(glib::Priority::DEFAULT);

// preview_tx is cloned into each spawned thread
// preview_rx.attach(None, callback) — callback runs on main GLib loop
```

The channel message type:

```rust
struct PreviewResult {
    id: ItemId,
    payload: Result<PreviewPayload, ()>,   // Err(()) → Failed
}
```

The `attach` callback receives a `PreviewResult`, mutates `state`, and calls
the corresponding item cell's update method.

## IPC channel

The IPC server thread also communicates back to the main thread, but it sends
raw file paths (not preview results). Use a separate `glib::MainContext::channel`:

```rust
let (ipc_tx, ipc_rx) = glib::MainContext::channel(glib::Priority::DEFAULT);
```

The `attach` callback for `ipc_rx` calls `ingestion::ingest(path, &state, ...)`.

## Thread summary

| Thread | What it does | Reports via |
|--------|-------------|-------------|
| Main (GLib loop) | GTK, state mutation, callbacks | — |
| IPC listener | Accept connections, read paths | `ipc_tx` glib channel |
| Preview worker (one per item) | Generate preview payload | `preview_tx` glib channel |

No thread pool for v1. One `std::thread::spawn` per preview item. At the scales
Purse operates at this is fine; revisit if startup with 50+ files feels slow.
