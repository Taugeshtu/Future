# Source File Map

Maps each source file to its responsibility and the concept spec section it implements.

## Cargo workspace root

```
Cargo.toml              workspace manifest; members = ["purse", "purse-niri"]
```

## `purse/src/`

| File | Responsibility | Concept ref |
|------|---------------|-------------|
| `main.rs` | Entry point: arg parsing, GTK Application init, wiring all subsystems | [[concept/lifecycle#purse-window-lifetime]], [[ipc-server]] |
| `state.rs` | `PurseState`, `Item`, `ItemId`, `PreviewState` — all owned data | [[concept/state]] |
| `ingestion.rs` | Path canonicalization, dedup check, MIME detection, preview task dispatch | [[concept/ingestion]] |
| `ipc.rs` | Unix socket server; receives paths, feeds ingestion | [[ipc-server]] |
| `preview.rs` | Per-MIME preview generation, runs in threads, reports via channel | [[concept/rendering#preview-per-mime-type]] |
| `layout.rs` | `gtk::FlowBox` setup and item insertion/layout | [[concept/rendering#grid-layout]] |
| `item_cell.rs` | Widget structure for a single grid cell; reflects item state | [[concept/rendering#preview-lifecycle-in-the-widget]], [[concept/state#hover]] |
| `interactions.rs` | `ShortcutController`, click handlers, hover tracking, clipboard | [[concept/interactions]] |
| `dispatch.rs` | Opens selected files via GIO; the v1 dispatch action | [[dispatch]] |

## `purse-niri/src/`

| File | Responsibility | Concept ref |
|------|---------------|-------------|
| `main.rs` | Entry point: spawn/locate instance and print target PID | [[concept/lifecycle#purse-niri-invocation-path]] |
| `niri.rs` | Niri IPC client: connect, query workspaces + windows | [[concept/purse-niri#niri-ipc-client]] |
| `resolution.rs` | Decision tree: find existing Purse on focused workspace, or decide to spawn | [[concept/purse-niri#instance-resolution-and-pid-output]] |
