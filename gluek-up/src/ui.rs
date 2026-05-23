use std::path::Path;
use gtk4::prelude::*;
use gtk4_layer_shell::{KeyboardMode, Layer, LayerShell};
use sourceview5::prelude::*;
use crate::MatchItem;

pub fn run(matches: Vec<MatchItem>, is_refs: bool) {
    let app = gtk4::Application::new(Some("games.tau.gluekup"), Default::default());
    
    app.connect_activate(move |application| {
        let window = gtk4::ApplicationWindow::new(application);
        
        // Initialize wayland layer shell
        window.init_layer_shell();
        window.set_layer(Layer::Overlay);
        window.set_keyboard_mode(KeyboardMode::Exclusive);
        
        // Centered layout: unset all anchors
        window.set_anchor(gtk4_layer_shell::Edge::Top, false);
        window.set_anchor(gtk4_layer_shell::Edge::Bottom, false);
        window.set_anchor(gtk4_layer_shell::Edge::Left, false);
        window.set_anchor(gtk4_layer_shell::Edge::Right, false);
        
        window.set_default_size(900, 500);
        window.set_title(Some(if is_refs { "Peek References" } else { "Peek Definition" }));
        
        // Set premium styling via CSS
        let provider = gtk4::CssProvider::new();
        provider.load_from_data("
            window {
                background-color: #0f172a;
                color: #f1f5f9;
                border-radius: 16px;
                border: 1px solid rgba(99, 102, 241, 0.4);
            }
            
            .header-box {
                background-color: #1e293b;
                padding: 12px 16px;
                border-top-left-radius: 16px;
                border-top-right-radius: 16px;
                border-bottom: 1px solid rgba(99, 102, 241, 0.2);
            }
            
            .header-title {
                font-size: 16px;
                font-weight: bold;
                color: #818cf8;
            }
            
            .main-paned {
                background-color: transparent;
            }
            
            .list-scroll {
                border-right: 1px solid rgba(255, 255, 255, 0.05);
            }
            
            .match-item {
                padding: 10px 14px;
                border-bottom: 1px solid rgba(255, 255, 255, 0.03);
            }
            
            listbox {
                background-color: transparent;
            }
            
            listbox row {
                background-color: transparent;
                transition: background-color 0.15s ease;
            }
            
            listbox row:hover {
                background-color: rgba(99, 102, 241, 0.1);
            }
            
            listbox row:selected {
                background-color: rgba(99, 102, 241, 0.25);
                border-left: 4px solid #6366f1;
            }
            
            textview {
                background-color: #090d16;
                color: #cbd5e1;
                font-family: 'JetBrains Mono', 'Fira Code', 'Monospace', monospace;
                font-size: 13px;
                padding: 12px;
            }
        ");
        
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        
        let root_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        
        // Header
        let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        header_box.add_css_class("header-box");
        let header_title = gtk4::Label::new(Some(if is_refs { "Peek References" } else { "Peek Definition" }));
        header_title.add_css_class("header-title");
        header_box.append(&header_title);
        
        let header_hint = gtk4::Label::new(Some("·  Esc  close  ·  Enter  go to definition"));
        header_hint.set_opacity(0.4);
        header_hint.set_halign(gtk4::Align::End);
        header_hint.set_hexpand(true);
        header_box.append(&header_hint);
        
        root_box.append(&header_box);
        
        // Main Pane
        let paned = gtk4::Paned::new(gtk4::Orientation::Horizontal);
        paned.add_css_class("main-paned");
        paned.set_position(320);
        paned.set_wide_handle(true);
        paned.set_hexpand(true);
        paned.set_vexpand(true);
        
        // Left Match List
        let list_scroll = gtk4::ScrolledWindow::new();
        list_scroll.add_css_class("list-scroll");
        list_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        
        let listbox = gtk4::ListBox::new();
        listbox.set_selection_mode(gtk4::SelectionMode::Single);
        
        if matches.is_empty() {
            let row_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
            row_box.add_css_class("match-item");
            
            let label_title = gtk4::Label::new(Some("Nothing found, sorry"));
            label_title.set_halign(gtk4::Align::Start);
            
            let label_sub = gtk4::Label::new(Some("No definition or reference matches resolved."));
            label_sub.set_halign(gtk4::Align::Start);
            label_sub.set_opacity(0.6);
            
            row_box.append(&label_title);
            row_box.append(&label_sub);
            listbox.append(&row_box);
        } else {
            for item in &matches {
                let row_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
                row_box.add_css_class("match-item");
                
                let label_title = gtk4::Label::new(Some(&item.label));
                label_title.set_halign(gtk4::Align::Start);
                label_title.set_ellipsize(gtk4::pango::EllipsizeMode::End);
                
                let path = Path::new(&item.file_path);
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or(&item.file_path);
                
                let label_sub = gtk4::Label::new(Some(&format!("{}:{}  -  {}", file_name, item.line, item.snippet.trim())));
                label_sub.set_halign(gtk4::Align::Start);
                label_sub.set_opacity(0.6);
                label_sub.set_ellipsize(gtk4::pango::EllipsizeMode::End);
                
                row_box.append(&label_title);
                row_box.append(&label_sub);
                
                listbox.append(&row_box);
            }
        }
        
        list_scroll.set_child(Some(&listbox));
        paned.set_start_child(Some(&list_scroll));
        
        // Right Preview Pane
        let preview_scroll = gtk4::ScrolledWindow::new();
        preview_scroll.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
        
        let view_preview = sourceview5::View::new();
        view_preview.set_editable(false);
        view_preview.set_cursor_visible(true);
        view_preview.set_highlight_current_line(true);
        view_preview.set_show_line_numbers(true);
        view_preview.set_hexpand(true);
        view_preview.set_vexpand(true);
        
        preview_scroll.set_child(Some(&view_preview));
        paned.set_end_child(Some(&preview_scroll));
        
        if matches.is_empty() {
            view_preview.buffer().set_text("Verify that the lsp-broker is running and the command configured is available.\n\nFor Markdown files, make sure 'markdown-oxide' or 'marksman' is installed and configured in ~/.config/lsp-broker/config.toml.");
        }
        
        root_box.append(&paned);
        window.set_child(Some(&root_box));
        
        // Selection Change Handler
        let matches_select = matches.clone();
        let view_preview_select = view_preview.clone();
        listbox.connect_row_selected(move |_, row| {
            if let Some(row) = row {
                let idx = row.index() as usize;
                if idx < matches_select.len() {
                    let item = &matches_select[idx];
                    if let Ok(content) = std::fs::read_to_string(&item.file_path) {
                        let buffer = view_preview_select.buffer();
                        buffer.set_text(&content);
                        
                        let lang_mgr = sourceview5::LanguageManager::default();
                        if let Some(lang) = lang_mgr.guess_language(Some(&item.file_path), None) {
                            buffer
                                .downcast_ref::<sourceview5::Buffer>()
                                .unwrap()
                                .set_language(Some(&lang));
                        }
                        
                        let line_idx = item.line.saturating_sub(1);
                        if let Some(iter) = buffer.iter_at_line(line_idx as i32) {
                            buffer.place_cursor(&iter);
                        }
                        
                        // Scroll to mark
                        if let Some(mark) = buffer.mark("insert") {
                            view_preview_select.scroll_to_mark(&mark, 0.0, true, 0.5, 0.5);
                        }
                    } else {
                        view_preview_select.buffer().set_text(&format!("Error: Could not read file {}", item.file_path));
                    }
                }
            }
        });
        
        // Keys Handling
        let window_keys = window.clone();
        let listbox_keys = listbox.clone();
        let matches_keys = matches.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            use gtk4::gdk::Key;
            match key {
                Key::Escape => {
                    window_keys.close();
                    gtk4::glib::Propagation::Stop
                }
                Key::Return => {
                    if let Some(row) = listbox_keys.selected_row() {
                        let idx = row.index() as usize;
                        if idx < matches_keys.len() {
                            let item = &matches_keys[idx];
                            let editor_cmd = std::env::var("GLUEK_UP_EDITOR_CMD")
                                .unwrap_or_else(|_| "lite-xl".to_string());
                            let arg = format!("{}:{}:{}", item.file_path, item.line, item.column);
                            
                            let _ = std::process::Command::new(editor_cmd)
                                .arg(arg)
                                .spawn();
                            
                            window_keys.close();
                            gtk4::glib::Propagation::Stop
                        } else {
                            gtk4::glib::Propagation::Proceed
                        }
                    } else {
                        gtk4::glib::Propagation::Proceed
                    }
                }
                _ => gtk4::glib::Propagation::Proceed,
            }
        });
        window.add_controller(key_controller);
        
        window.present();
        
        // Focus first match immediately and trigger preview
        if let Some(first_row) = listbox.row_at_index(0) {
            listbox.select_row(Some(&first_row));
        }
        listbox.grab_focus();
    });
    
    app.run_with_args(&["gluek-up"]);
}
