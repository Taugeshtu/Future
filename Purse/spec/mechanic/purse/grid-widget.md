# Grid Widget

`layout.rs` — creates and configures the `gtk::FlowBox` that arranges item cells.

## FlowBox setup

```rust
pub fn build_grid() -> gtk::FlowBox {
    let flow_box = gtk::FlowBox::new();
    flow_box.set_selection_mode(gtk::SelectionMode::None); // we own selection
    flow_box.set_homogeneous(true);                        // uniform cell sizes
    flow_box.set_column_spacing(14);
    flow_box.set_row_spacing(14);
    flow_box.set_margin_top(12);
    flow_box.set_margin_bottom(12);
    flow_box.set_margin_start(12);
    flow_box.set_margin_end(12);
    flow_box.set_max_children_per_line(2);
    flow_box.set_min_children_per_line(1);
    flow_box
}
```

`SelectionMode::None` disables GTK's built-in selection visuals entirely.
Dismissal state is managed in `PurseState`; dismissed items are removed from the
FlowBox entirely rather than visually marked.

`max_children_per_line(2)` targets a 2-column layout at the default 900px window
width. `min_children_per_line(1)` allows graceful narrowing without overflow.

## Adding items

```rust
pub fn append_item(flow_box: &gtk::FlowBox, cell_widget: &gtk::Widget) {
    flow_box.append(cell_widget);
}
```

Called from ingestion after an `ItemCell` is constructed.
`FlowBox` wraps to a new row automatically when the window width is exceeded.

## Window container

FlowBox sits directly inside the `ApplicationWindow` (no `ScrolledWindow` in v1).
The window has a fixed default size; cells that don't fit are clipped.
An overflow label ("... and N more") is shown below the FlowBox when
`state.items.len()` exceeds what fits — this is handled in `main.rs` or a
thin wrapper, not in `layout.rs` itself.

Overflow threshold is not computed geometrically in v1 — use a fixed cap
(e.g. 24 items) as an approximation. Good enough; revisit if needed.

## Homogeneous sizing

All cells are the same size. FlowBox enforces this via `set_homogeneous(true)`.
Cell height is determined by `PREVIEW_HEIGHT` (from `preview.rs`) enforced on the
stack's `ScrolledWindow`. Cell width fills the available column width — item cells
use `set_hexpand(true)` all the way up through stack → root box, so FlowBox
stretches each cell to fill its column.
