mod dispatch;
mod ingestion;
mod interactions;
mod ipc;
mod item_cell;
mod layout;
mod preview;
mod state;
mod thumbnailer;

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{gio, glib};

use item_cell::ItemCell;
use state::{ItemId, PurseState};

fn main() {
    std::env::set_var("GSK_RENDERER", "cairo");
    dispatch::init();
    let initial_paths: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();

    let app = gtk4::Application::new(
        Some("dev.purse"),
        gio::ApplicationFlags::NON_UNIQUE,
    );

    app.connect_activate(move |app| {
        build_window(app, initial_paths.clone());
    });

    // Pass only argv[0] — we handle file arguments ourselves above.
    // Without this, GLib sees the paths and tries to open them via GIO's
    // HANDLES_OPEN mechanism, which we don't use.
    let argv0: Vec<String> = std::env::args().take(1).collect();
    let exit_code = app.run_with_args(&argv0);

    // clean up socket on normal exit
    let _ = std::fs::remove_file(ipc::socket_path());

    std::process::exit(exit_code.into());
}

fn build_window(app: &gtk4::Application, initial_paths: Vec<PathBuf>) {
    // --- channels ---
    let (ipc_tx, ipc_rx) = async_channel::unbounded::<ipc::IpcMessage>();
    let (preview_tx, preview_rx) = async_channel::unbounded::<state::PreviewResult>();

    // --- shared state ---
    let state: Rc<RefCell<PurseState>> = Rc::new(RefCell::new(PurseState::new()));
    let cell_map: Rc<RefCell<HashMap<ItemId, ItemCell>>> = Rc::new(RefCell::new(HashMap::new()));

    // --- window ---
    let window = gtk4::ApplicationWindow::new(app);
    window.set_default_size(900, 700);
    window.set_title(Some(&format!("purse-{}", std::process::id())));

    // --- CSS ---
    load_css();

    // --- layout ---
    let flow_box = layout::build_grid();
    let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    vbox.add_css_class("purse-root");
    vbox.append(&flow_box);
    window.set_child(Some(&vbox));

    // --- IPC rx: feed messages into ingestion ---
    glib::MainContext::default().spawn_local({
        let state = state.clone();
        let cell_map = cell_map.clone();
        let flow_box = flow_box.clone();
        let preview_tx = preview_tx.clone();
        async move {
            while let Ok(msg) = ipc_rx.recv().await {
                ingestion::ingest(msg, &state, &cell_map, &flow_box, preview_tx.clone());
            }
        }
    });

    // --- preview rx: update cells when previews arrive ---
    glib::MainContext::default().spawn_local({
        let state = state.clone();
        let cell_map = cell_map.clone();
        async move {
            while let Ok(result) = preview_rx.recv().await {
                let payload = match result.payload {
                    Ok(p) => p,
                    Err(()) => preview::PreviewPayload::Icon {
                        name: "text-x-generic".to_string(),
                    },
                };
                state.borrow_mut().items[result.id].preview =
                    state::PreviewState::Ready(payload.clone());
                if let Some(cell) = cell_map.borrow().get(&result.id) {
                    cell.set_preview(&payload);
                }
            }
        }
    });

    // --- keyboard shortcuts ---
    interactions::setup_shortcuts(&window, state.clone(), cell_map.clone());

    // --- drag and drop ---
    {
        let state = state.clone();
        let cell_map = cell_map.clone();
        let flow_box = flow_box.clone();
        let preview_tx = preview_tx.clone();
        interactions::setup_dnd(&window, move |path| {
            ingestion::ingest(ipc::IpcMessage::File(path), &state, &cell_map, &flow_box, preview_tx.clone());
        });
    }

    // --- IPC server ---
    ipc::spawn_server(ipc_tx);
    println!("READY");
    let _ = std::io::Write::flush(&mut std::io::stdout());

    #[cfg(unix)]
    unsafe {
        extern "C" {
            fn open(pathname: *const u8, flags: i32) -> i32;
            fn dup2(oldfd: i32, newfd: i32) -> i32;
            fn close(fd: i32) -> i32;
        }
        let fd = open(b"/dev/null\0".as_ptr(), 1); // 1 is O_WRONLY
        if fd >= 0 {
            dup2(fd, 1);
            close(fd);
        }
    }

    // --- ingest initial files from argv ---
    for path in initial_paths {
        ingestion::ingest(ipc::IpcMessage::File(path), &state, &cell_map, &flow_box, preview_tx.clone());
    }

    window.present();
}

fn load_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_string(
        r#"
        window {
            background-color: transparent;
        }
        .purse-root {
            background-color: rgba(255, 255, 255, 0.4);
            border-radius: 8px;
            padding: 8px;
        }
        .purse-item {
            border: 10px solid transparent;
            border-radius: 4px;
        }
        .purse-hovered {
            border: 10px solid #87ceeb;
        }
        "#,
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("no display"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
