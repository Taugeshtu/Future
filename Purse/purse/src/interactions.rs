use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::prelude::*;

use crate::dispatch;
use crate::item_cell::ItemCell;
use crate::state::{ItemId, PurseState};

pub fn setup_shortcuts(
    window: &gtk4::ApplicationWindow,
    state: Rc<RefCell<PurseState>>,
    _cell_map: Rc<RefCell<HashMap<ItemId, ItemCell>>>,
) {
    // Enter — capture phase so FlowBox doesn't eat it
    {
        let state = state.clone();
        let window_for_close = window.clone();
        let window_for_ctrl = window.clone();
        let key_ctrl = gtk4::EventControllerKey::new();
        key_ctrl.set_propagation_phase(gtk4::PropagationPhase::Capture);
        key_ctrl.connect_key_pressed(move |_, keyval, _, _| {
            if keyval == gtk4::gdk::Key::Return {
                let paths: Vec<PathBuf> = state
                    .borrow()
                    .items
                    .iter()
                    .filter(|i| i.selected)
                    .map(|i| i.path.clone())
                    .collect();
                dispatch::open_files_bypass_self(&paths);
                window_for_close.close();
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        window_for_ctrl.add_controller(key_ctrl);
    }

    let controller = gtk4::ShortcutController::new();
    controller.set_scope(gtk4::ShortcutScope::Global);

    // Escape — close window
    {
        let window = window.clone();
        let action = gtk4::CallbackAction::new(move |_, _| {
            window.close();
            glib::Propagation::Proceed
        });
        let trigger = gtk4::ShortcutTrigger::parse_string("Escape").unwrap();
        controller.add_shortcut(gtk4::Shortcut::new(Some(trigger), Some(action)));
    }

    // Ctrl+C — copy hovered item's full content to clipboard
    {
        let state = state.clone();
        let action = gtk4::CallbackAction::new(move |widget, _| {
            let hovered_path = {
                let s = state.borrow();
                s.hover
                    .and_then(|id| s.items.get(id))
                    .map(|i| i.path.clone())
            };
            if let Some(path) = hovered_path {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Some(display) = gtk4::gdk::Display::default() {
                        display.clipboard().set_text(&content);
                    }
                }
            }
            // get the widget's display for clipboard — use the widget parameter
            let _ = widget;
            glib::Propagation::Proceed
        });
        let trigger = gtk4::ShortcutTrigger::parse_string("<Control>c").unwrap();
        controller.add_shortcut(gtk4::Shortcut::new(Some(trigger), Some(action)));
    }

    window.add_controller(controller);
}

pub fn setup_dnd(
    window: &gtk4::ApplicationWindow,
    ingest: impl Fn(PathBuf) + 'static,
) {
    let target = gtk4::DropTarget::new(
        gtk4::gdk::FileList::static_type(),
        gtk4::gdk::DragAction::COPY,
    );
    target.connect_drop(move |_, value, _, _| {
        if let Ok(file_list) = value.get::<gtk4::gdk::FileList>() {
            for file in file_list.files() {
                if let Some(path) = file.path() {
                    ingest(path);
                }
            }
        }
        true
    });
    window.add_controller(target);
}
