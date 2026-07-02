use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use async_channel::Sender;
use gtk4::prelude::*;

use crate::item_cell::ItemCell;
use crate::layout;
use crate::preview;
use crate::state::{Item, ItemId, PreviewResult, PreviewState, PurseState};

pub fn ingest(
    msg: crate::ipc::IpcMessage,
    state: &Rc<RefCell<PurseState>>,
    cell_map: &Rc<RefCell<HashMap<ItemId, ItemCell>>>,
    flow_box: &gtk4::FlowBox,
    preview_tx: Sender<PreviewResult>,
) {
    // 1. Find existing item ID and resolve details if file
    let mut resolved_file = None;
    
    let (existing_id, is_transient_delete) = {
        let items = &state.borrow().items;
        match &msg {
            crate::ipc::IpcMessage::File(path) => {
                let mut clean_path = path.clone();
                let (mut line, mut col) = (None, None);
                let s = path.to_string_lossy();
                let p: Vec<&str> = s.split(':').collect();
                if p.len() >= 2 {
                    if let Ok(n1) = p[p.len() - 1].parse::<u32>() {
                        if p.len() >= 3 && p[p.len() - 2].parse::<u32>().is_ok() {
                            line = Some(p[p.len() - 2].parse().unwrap());
                            col = Some(n1);
                            clean_path = PathBuf::from(p[..p.len() - 2].join(":"));
                        } else {
                            line = Some(n1);
                            clean_path = PathBuf::from(p[..p.len() - 1].join(":"));
                        }
                    }
                }
                let canonical = match std::fs::canonicalize(&clean_path) {
                    Ok(p) => p,
                    Err(_) => return,
                };
                let id = items.iter().find_map(|i| match &i.kind {
                    crate::state::ItemKind::File { path: p, .. } if p == &canonical => Some(i.id),
                    _ => None,
                });
                resolved_file = Some((canonical, line, col));
                (id, false)
            }
            crate::ipc::IpcMessage::Transient(payload) => {
                let id = items.iter().find_map(|i| match &i.kind {
                    crate::state::ItemKind::Transient { uuid, .. } if uuid == &payload.uuid => Some(i.id),
                    _ => None,
                });
                (id, payload.content.is_empty())
            }
        }
    };

    // 2. If item already exists in the state
    if let Some(id) = existing_id {
        if is_transient_delete {
            state.borrow_mut().items[id].selected = false;
            if let Some(cell) = cell_map.borrow().get(&id) {
                if let Some(fb_child) = cell.widget().parent() {
                    fb_child.unparent();
                }
            }
            cell_map.borrow_mut().remove(&id);
        } else if let crate::ipc::IpcMessage::Transient(payload) = &msg {
            state.borrow_mut().items[id].kind = crate::state::ItemKind::Transient {
                uuid: payload.uuid.clone(),
                label: payload.label.clone(),
                content: payload.content.clone(),
            };
            if let Some(cell) = cell_map.borrow().get(&id) {
                cell.update_transient(&payload.label, &payload.content);
            }
        }
        return;
    }

    // 3. If item does not exist, and it's a delete request, ignore
    if is_transient_delete {
        return;
    }

    // 4. Create new item
    let id = state.borrow().items.len();
    let (kind, preview_state) = match &msg {
        crate::ipc::IpcMessage::File(_) => {
            let (canonical, line, col) = resolved_file.unwrap();
            let sniff: Vec<u8> = {
                use std::io::Read;
                let mut buf = [0u8; 512];
                let n = std::fs::File::open(&canonical)
                    .and_then(|mut f| f.read(&mut buf))
                    .unwrap_or(0);
                buf[..n].to_vec()
            };
            let (mime, _uncertain) = gtk4::gio::functions::content_type_guess(
                Some(canonical.to_str().unwrap_or("")),
                &sniff,
            );
            let mime = mime.to_string();
            
            (
                crate::state::ItemKind::File {
                    path: canonical,
                    mime,
                    line,
                    col,
                },
                PreviewState::Pending,
            )
        }
        crate::ipc::IpcMessage::Transient(payload) => (
            crate::state::ItemKind::Transient {
                uuid: payload.uuid.clone(),
                label: payload.label.clone(),
                content: payload.content.clone(),
            },
            PreviewState::Ready(crate::preview::PreviewPayload::Text {
                content: payload.content.clone(),
            }),
        ),
    };

    let item = Item {
        id,
        preview: preview_state,
        selected: true,
        kind: kind.clone(),
    };
    state.borrow_mut().items.push(item);

    // 5. Create cell and append to grid
    let cell = ItemCell::new(id, state.clone(), cell_map.clone());
    layout::append_item(flow_box, cell.widget());
    cell_map.borrow_mut().insert(id, cell);

    // 6. Spawn preview if file
    if let crate::state::ItemKind::File { path, mime, line, .. } = kind {
        std::thread::spawn(move || {
            let result = preview::generate(&path, &mime, line);
            preview_tx.send_blocking(PreviewResult { id, payload: result }).ok();
        });
    }
}
