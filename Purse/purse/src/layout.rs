use gtk4::prelude::*;

pub fn build_grid() -> gtk4::FlowBox {
    let flow_box = gtk4::FlowBox::new();
    flow_box.set_selection_mode(gtk4::SelectionMode::None);
    flow_box.set_homogeneous(true);
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

pub fn append_item(flow_box: &gtk4::FlowBox, widget: &gtk4::Widget) {
    flow_box.append(widget);
}
