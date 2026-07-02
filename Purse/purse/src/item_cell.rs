use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::prelude::*;
use sourceview5::prelude::*;

use crate::dispatch;
use crate::preview::{PreviewPayload, PREVIEW_HEIGHT};
use crate::state::{ItemId, PurseState};

pub struct ItemCell {
    pub id: ItemId,
    path: Option<PathBuf>,
    root: gtk4::Box,
    stack: gtk4::Stack,
    text_view: sourceview5::View,
    picture: gtk4::Picture,
    icon: gtk4::Image,
    spinner: gtk4::Spinner,
    label: gtk4::Label,
}

impl ItemCell {
    pub fn new(
        id: ItemId,
        state: Rc<RefCell<PurseState>>,
        cell_map: Rc<RefCell<HashMap<ItemId, ItemCell>>>,
    ) -> Self {
        let kind = {
            let s = state.borrow();
            s.items[id].kind.clone()
        };

        // --- preview widgets ---
        let spinner = gtk4::Spinner::new();
        if let crate::state::ItemKind::File { .. } = &kind {
            spinner.start();
        }

        let text_view = sourceview5::View::new();
        text_view.set_editable(false);
        text_view.set_cursor_visible(false);
        text_view.set_hexpand(true);
        text_view.set_vexpand(true);

        // ScrolledWindow clips the text view to a fixed height.
        // set_propagate_natural_height(false) is the key: without it the
        // ScrolledWindow just reports the child's natural height as its own,
        // making size_request irrelevant. With it, min_content_height wins.
        let text_scroll = gtk4::ScrolledWindow::new();
        text_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Never);
        text_scroll.set_propagate_natural_height(false);
        text_scroll.set_min_content_height(PREVIEW_HEIGHT);
        text_scroll.set_hexpand(true);
        text_scroll.set_child(Some(&text_view));

        let picture = gtk4::Picture::new();
        picture.set_size_request(-1, PREVIEW_HEIGHT);
        picture.set_hexpand(true);

        let icon = gtk4::Image::new();
        icon.set_pixel_size(64);
        icon.set_size_request(-1, PREVIEW_HEIGHT);
        icon.set_hexpand(true);

        // --- stack ---
        let stack = gtk4::Stack::new();
        stack.add_named(&spinner, Some("pending"));
        stack.add_named(&text_scroll, Some("text"));
        stack.add_named(&picture, Some("image"));
        stack.add_named(&icon, Some("icon"));

        let (initial_stack_child, path_field) = match &kind {
            crate::state::ItemKind::File { path, .. } => {
                ("pending", Some(path.clone()))
            }
            crate::state::ItemKind::Transient { content, .. } => {
                let buffer = text_view.buffer();
                buffer.set_text(content);
                ("text", None)
            }
        };
        stack.set_visible_child_name(initial_stack_child);
        stack.set_size_request(-1, PREVIEW_HEIGHT);
        stack.set_hexpand(true);
        stack.set_vexpand(false);

        // --- filename label ---
        let display_label = match &kind {
            crate::state::ItemKind::File { path, .. } => {
                path.file_name().and_then(|n| n.to_str()).map(|s| s.to_string())
            }
            crate::state::ItemKind::Transient { label, .. } => {
                Some(label.clone())
            }
        };
        let label = gtk4::Label::new(display_label.as_deref());
        label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        label.set_max_width_chars(1); // let the container control width
        label.set_hexpand(true);

        // --- root box ---
        let root = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        root.set_hexpand(true);
        root.append(&stack);
        root.append(&label);

        root.add_css_class("purse-item");

        let cell = ItemCell {
            id,
            path: path_field,
            root,
            stack,
            text_view,
            picture,
            icon,
            spinner,
            label,
        };

        // --- hover ---
        {
            let state_enter = state.clone();
            let state_leave = state.clone();
            let root_enter = cell.root.clone();
            let root_leave = cell.root.clone();
            let motion = gtk4::EventControllerMotion::new();
            motion.connect_enter(move |_, _, _| {
                state_enter.borrow_mut().hover = Some(id);
                root_enter.add_css_class("purse-hovered");
            });
            motion.connect_leave(move |_| {
                let mut s = state_leave.borrow_mut();
                if s.hover == Some(id) {
                    s.hover = None;
                }
                root_leave.remove_css_class("purse-hovered");
            });
            cell.root.add_controller(motion);
        }

        // --- click: single = remove item, double = open in real handler ---
        {
            let state_click = state.clone();
            let cell_map_click = cell_map.clone();
            let root_click = cell.root.clone();
            let click = gtk4::GestureClick::new();
            click.connect_released(move |_, n_press, _, _| {
                if n_press >= 2 {
                    let item = &state_click.borrow().items[id];
                    if let crate::state::ItemKind::File { path, line, col, .. } = &item.kind {
                        dispatch::open_files_bypass_self(&[(path.clone(), *line, *col)]);
                    }
                } else {
                    state_click.borrow_mut().items[id].selected = false;
                    cell_map_click.borrow_mut().remove(&id);
                    if let Some(fb_child) = root_click.parent() {
                        fb_child.unparent();
                    }
                }
            });
            cell.root.add_controller(click);
        }

        cell
    }

    pub fn widget(&self) -> &gtk4::Widget {
        self.root.upcast_ref()
    }

    pub fn set_preview(&self, payload: &PreviewPayload) {
        match payload {
            PreviewPayload::Text { content } => {
                let buffer = self.text_view.buffer();
                // language detection on the main thread
                let lang_mgr = sourceview5::LanguageManager::default();
                if let Some(path) = &self.path {
                    if let Some(lang) = lang_mgr.guess_language(
                        Some(path.to_str().unwrap_or("")),
                        None,
                    ) {
                        buffer
                            .downcast_ref::<sourceview5::Buffer>()
                            .unwrap()
                            .set_language(Some(&lang));
                    }
                }
                buffer.set_text(content);
                self.stack.set_visible_child_name("text");
                self.label.set_visible(false);
            }
            PreviewPayload::Image(path) => {
                let file = gtk4::gio::File::for_path(path);
                self.picture.set_file(Some(&file));
                self.stack.set_visible_child_name("image");
                self.label.set_visible(false);
            }
            PreviewPayload::Icon { name } => {
                self.icon.set_icon_name(Some(name));
                self.stack.set_visible_child_name("icon");
                self.label.set_visible(true);
            }
        }
        self.spinner.stop();
    }

    pub fn update_transient(&self, label: &str, content: &str) {
        self.label.set_text(label);
        let buffer = self.text_view.buffer();
        buffer.set_text(content);
        self.stack.set_visible_child_name("text");
    }
}
