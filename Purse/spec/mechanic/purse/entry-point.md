# Entry Point

`main.rs` — wires all subsystems and hands control to the GLib main loop.

## Sequence

```
1. parse argv → Vec<PathBuf> (initial file list; may be empty)

2. create glib channels:
     ipc_tx / ipc_rx      (IPC server → ingestion)
     preview_tx / preview_rx  (preview threads → item cell update)

3. gtk::Application::new("dev.purse", gio::ApplicationFlags::NON_UNIQUE)
   NON_UNIQUE: multiple Purse windows are allowed (one per workspace);
   instance resolution is purse-niri's job, not ours.

4. connect "activate":
     a. create PurseState, wrap in Rc<RefCell<>>
     b. build ApplicationWindow
          - default size: 900 × 700 (enough for ~12 cells at default preview size)
          - title: format!("purse-{}", std::process::id())
            ↑ this is how purse-niri finds our PID via Niri's window list
          - app-id already set on Application; window inherits it
     c. build grid (grid.rs) — returns FlowBox + ScrolledWindow/container
     d. attach ipc_rx callback → calls ingest() per received path
     e. attach preview_rx callback → calls item_cell.set_preview() + state update
     f. setup ShortcutController (interactions.rs)
     g. setup DnD DropTarget on the window (interactions.rs)
     h. spawn IPC server thread with ipc_tx (ipc.rs)
     i. ingest initial files from argv (ingestion.rs)
     j. window.present()

5. application.run()
```

## Window title convention

Title is set to `purse-{pid}` (e.g. `"purse-12345"`).

`purse-niri` parses PID from this title when resolving instances.
This is a deliberate encoding; do not change the format without updating
`purse-niri/resolution.rs`.

In niri, the title bar can be suppressed via a window rule matching
`app-id="dev.purse"`. That is a user configuration concern, not ours.

## NON_UNIQUE flag

GTK's default is to route second-instance launches to the first instance's
`activate` signal. We do not want this — each `purse` invocation is an
independent window. `NON_UNIQUE` disables the single-instance behaviour.
