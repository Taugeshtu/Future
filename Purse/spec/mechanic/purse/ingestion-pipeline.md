# Ingestion Pipeline

`ingestion.rs` — turns a raw path into a live item in state and a cell in the grid.

## Steps

```
receive PathBuf
    │
    ▼
1. canonicalize
   std::fs::canonicalize(path)
   on error (path doesn't exist, permission denied): log and drop silently

    │
    ▼
2. dedup check
   state.borrow().items.iter().any(|i| i.path == canonical)
   if found: return early, do nothing

    │
    ▼
3. MIME detection
   Read first 512 bytes of the file, pass as content hint:
   gio::functions::content_type_guess(Some(&path), &sniff[..n])
   returns (content_type: String, uncertain: bool)
   uncertain flag is informational only; we use the result regardless

   ⚠️ Do NOT pass &[] (empty data). On systems where the file extension is not
   in the local mime-db (e.g. .png missing from a minimal install), GIO falls
   back to "application/x-zerosize" which breaks both preview routing and the
   D-Bus thumbnailer (tumbler silently drops requests for unknown MIME types).

    │
    ▼
4. create Item
   let id = state.borrow().items.len();   // index = id, append-only
   Item { id, path: canonical, mime, preview: PreviewState::Pending, selected: true }

    │
    ▼
5. push to state
   state.borrow_mut().items.push(item);

    │
    ▼
6. append cell to grid
   let cell = ItemCell::new(id, &path, &state);
   flow_box.append(cell.widget());
   cell_map.insert(id, cell);             // cell_map: HashMap<ItemId, ItemCell>

    │
    ▼
7. spawn preview task
   let tx = preview_tx.clone();
   let path = canonical.clone();
   let mime = mime.clone();
   std::thread::spawn(move || {
       let result = preview::generate(&path, &mime);
       tx.send(PreviewResult { id, payload: result }).ok();
   });
```

## `cell_map`

`HashMap<ItemId, ItemCell>` lives alongside `PurseState` in `main.rs`
(also wrapped in `Rc<RefCell<>>`). It is the bridge between async preview
results and the widgets that need updating.

Not part of `PurseState` itself — it holds GTK widgets, and state is GTK-free.

## Error handling philosophy

Ingestion errors (bad path, unreadable file) are silent drops in v1.
No error state in the UI. If a file can't be previewed, it shows icon fallback.
If a path can't be canonicalized, it simply never appears. This is acceptable:
the user sent a bad path somehow; Purse is not a validator.
