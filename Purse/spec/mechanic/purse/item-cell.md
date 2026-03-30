# Item Cell

`item_cell.rs` — the widget for a single item in the grid.

## Structure

```
gtk::Box (vertical, hexpand)
├── preview_stack: gtk::Stack  (hexpand, fixed height = PREVIEW_HEIGHT)
│   ├── "pending"  → gtk::Spinner (animating)
│   ├── "text"     → gtk::ScrolledWindow (clips text_view to PREVIEW_HEIGHT)
│   │                  └── sourceview5::View (read-only)
│   ├── "image"    → gtk::Picture (hexpand)
│   └── "icon"     → gtk::Image (themed icon, large, hexpand)
└── gtk::Label (filename, ellipsized, single line)
     — visible only while preview is pending; hidden on set_preview()
```

`gtk::Stack` swaps the preview widget in place without layout thrash.
Initial visible child: `"pending"`.

The `ScrolledWindow` around the text view uses `set_propagate_natural_height(false)`
so it doesn't report the text view's natural height as its own — this is what
actually enforces the fixed preview height. `PolicyType::Never` on both axes
means no scrollbars; content beyond the height is clipped.

## Rust struct

```rust
pub struct ItemCell {
    pub id: ItemId,
    path: PathBuf,
    root: gtk::Box,
    stack: gtk::Stack,
    text_view: sourceview5::View,
    picture: gtk::Picture,
    icon: gtk::Image,
    spinner: gtk::Spinner,
    label: gtk::Label,
}
```

`ItemCell::widget()` returns `self.root.upcast_ref::<gtk::Widget>()` for FlowBox insertion.

## Construction

```rust
impl ItemCell {
    pub fn new(
        id: ItemId,
        path: &Path,
        state: Rc<RefCell<PurseState>>,
        cell_map: Rc<RefCell<HashMap<ItemId, ItemCell>>>,
    ) -> Self
```

Spinner starts animating immediately. Label shows the filename.
All controllers (hover, click) are attached here.

## Preview update

Called from the `preview_rx` glib channel callback in `main.rs`:

```rust
pub fn set_preview(&self, payload: &PreviewPayload) {
    match payload {
        PreviewPayload::Text { content } => {
            let lang_mgr = sourceview5::LanguageManager::default();
            if let Some(lang) = lang_mgr.guess_language(Some(self.path.to_str()), None) {
                self.text_view.buffer()
                    .downcast_ref::<sourceview5::Buffer>().unwrap()
                    .set_language(Some(&lang));
            }
            self.text_view.buffer().set_text(content);
            self.stack.set_visible_child_name("text");
        }
        PreviewPayload::Image(path) => {
            self.picture.set_file(Some(&gio::File::for_path(path)));
            self.stack.set_visible_child_name("image");
        }
        PreviewPayload::Icon { name } => {
            self.icon.set_icon_name(Some(name));
            self.stack.set_visible_child_name("icon");
        }
    }
    self.spinner.stop();
    self.label.set_visible(false);  // filename only needed during pending
}
```

On `PreviewState::Failed`: `set_preview` is called with a generic icon payload
(`"text-x-generic"`), so the label is hidden even on failure.

## Hover controllers

One `gtk::EventControllerMotion` per cell:

```rust
motion.connect_enter(move |_, _, _| {
    state.borrow_mut().hover = Some(id);
    root.add_css_class("purse-hovered");
});
motion.connect_leave(move |_| {
    let mut s = state.borrow_mut();
    if s.hover == Some(id) { s.hover = None; }
    root.remove_css_class("purse-hovered");
});
```

## Click controller

Single-click dismisses the item (removes it from the visible list).
Double-click opens the file in its real handler without closing Purse.

```rust
click.connect_released(move |_, n_press, _, _| {
    if n_press >= 2 {
        dispatch::open_files_bypass_self(&[path.clone()]);
    } else {
        state.borrow_mut().items[id].selected = false;
        cell_map.borrow_mut().remove(&id);
        if let Some(fb_child) = root.parent() {
            fb_child.unparent();   // removes FlowBoxChild from FlowBox
        }
    }
});
```

`root.parent()` is the `FlowBoxChild` wrapper GTK auto-creates on `flow_box.append()`.
Calling `unparent()` on it cleanly removes it without needing a FlowBox reference.

Note the explicit ordering: state mutation, then cell_map removal, then widget removal.
No `RefCell` double-borrow risk because the borrows are released between steps.
