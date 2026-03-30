# Interaction Handling

`interactions.rs` — keyboard shortcuts and drag-and-drop.
Click and hover handlers live in `item_cell.rs`; this file handles window-level input.

## Key handling architecture

Enter and Escape/Ctrl+C use different mechanisms due to a GTK4 event-propagation
quirk: `ShortcutController` with `ShortcutScope::Global` does NOT reliably catch
Enter when a `FlowBox` has focus — FlowBox consumes Enter in the bubble phase before
the shortcut controller sees it.

**Enter** — `EventControllerKey` attached to the window at capture phase:
```rust
let key_ctrl = gtk::EventControllerKey::new();
key_ctrl.set_propagation_phase(gtk::PropagationPhase::Capture);
key_ctrl.connect_key_pressed(move |_, keyval, _, _| {
    if keyval == gdk::Key::Return {
        // dispatch + close
        return glib::Propagation::Stop;
    }
    glib::Propagation::Proceed
});
window.add_controller(key_ctrl);
```

Capture phase fires before any child widget sees the event, so FlowBox can't eat it.

**Escape and Ctrl+C** — `ShortcutController` with `ShortcutScope::Global` (these
keys aren't consumed by FlowBox so the simpler mechanism works fine):
```rust
let controller = gtk::ShortcutController::new();
controller.set_scope(gtk::ShortcutScope::Global);
// add Escape → close, <Control>c → copy hovered
window.add_controller(controller);
```

## Enter — dispatch

```rust
let paths: Vec<PathBuf> = state.borrow().items.iter()
    .filter(|i| i.selected)   // selected=false means dismissed (tombstone)
    .map(|i| i.path.clone())
    .collect();
dispatch::open_files_bypass_self(&paths);
window.close();
```

## Escape — close

```rust
fn close_action(window: &gtk::ApplicationWindow) -> impl Fn() {
    let w = window.clone();
    move || w.close()
}
```

## Ctrl+C — copy hovered item content

```rust
fn copy_action(state: Rc<RefCell<PurseState>>) -> impl Fn() {
    move || {
        let hovered_path = {
            let s = state.borrow();
            s.hover.and_then(|id| s.items.get(id)).map(|i| i.path.clone())
        };
        if let Some(path) = hovered_path {
            if let Ok(content) = std::fs::read_to_string(&path) {
                // gdk::Display::default() is available on main thread
                if let Some(clipboard) = gdk::Display::default().map(|d| d.clipboard()) {
                    clipboard.set_text(&content);
                }
            }
        }
        // if no item hovered, or file unreadable: silently do nothing
    }
}
```

Reads the file fresh on Ctrl+C rather than using the cached preview content.
Rationale: preview truncates to 20 lines; clipboard should get the full file.
This is a deliberate divergence from the preview payload.

## Drag and drop

```rust
pub fn setup_dnd(
    window: &gtk::ApplicationWindow,
    ingest: impl Fn(PathBuf) + 'static,
) {
    let target = gtk::DropTarget::new(gdk::FileList::static_type(), gdk::DragAction::COPY);
    target.connect_drop(move |_, value, _, _| {
        if let Ok(file_list) = value.get::<gdk::FileList>() {
            for file in file_list.files() {
                if let Some(path) = file.path() {
                    ingest(path);
                }
            }
        }
        true // accept the drop
    });
    window.add_controller(target);
}
```

`ingest` is a closure from `main.rs` that captures `state`, `cell_map`, and
`preview_tx` and calls into `ingestion::ingest(...)`.
