# State Types

All owned data. No GTK types here.

## `ItemId`

```rust
type ItemId = usize;
```

Index into `PurseState::items`. Stable for the lifetime of the process.
`items` is append-only at the Vec level — entries are never physically removed,
because that would invalidate subsequent IDs. Dismissed items become tombstones
(`selected = false`); their widget and cell_map entry are dropped, but the Vec
slot stays.

## `PreviewState`

```rust
enum PreviewState {
    Pending,
    Ready(PreviewPayload),
    Failed,
}

enum PreviewPayload {
    Text { content: String },
    Image(PathBuf),   // path to the on-disk thumbnail
    Icon { name: String },
}
```

`Image` carries the thumbnail file path, not a `Pixbuf`. The cell loads it via
`gtk4::Picture::set_file`. This avoids holding a decoded image in memory after
the cell has been populated.

Language detection for text previews is done in `item_cell::set_preview` on the
main thread (using `SourceView::LanguageManager::guess_language`), not during
background preview generation. `PreviewPayload::Text` carries only raw content.

## `Item`

```rust
struct Item {
    id: ItemId,
    path: PathBuf,          // canonical absolute path; identity key for dedup
    mime: String,           // GIO content type string, e.g. "text/x-rust"
    preview: PreviewState,  // starts Pending; updated async
    selected: bool,         // starts true; false = dismissed (tombstone)
}
```

## `PurseState`

```rust
struct PurseState {
    items: Vec<Item>,
    hover: Option<ItemId>,
}
```

`items` Vec is append-only (tombstone model, see ItemId above). Index = ItemId.
`hover` is `None` when pointer is outside all cells.

No socket handle here. The IPC server owns its socket independently;
it feeds paths into ingestion via a channel, not via direct state mutation.
