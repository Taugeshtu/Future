use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use async_channel::Sender;

use crate::item_cell::ItemCell;
use crate::layout;
use crate::preview;
use crate::state::{Item, ItemId, PreviewResult, PreviewState, PurseState};

pub fn ingest(
    path: PathBuf,
    state: &Rc<RefCell<PurseState>>,
    cell_map: &Rc<RefCell<HashMap<ItemId, ItemCell>>>,
    flow_box: &gtk4::FlowBox,
    preview_tx: Sender<PreviewResult>,
) {
    // 1. canonicalize
    let canonical = match std::fs::canonicalize(&path) {
        Ok(p) => p,
        Err(_) => return,
    };

    // 2. dedup
    if state.borrow().items.iter().any(|i| i.path == canonical) {
        return;
    }

    // 3. MIME detection — sniff content bytes so GIO doesn't fall back to
    //    application/x-zerosize when the extension isn't in the local mime-db
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

    // 4 & 5. create item and push to state
    let id = state.borrow().items.len();
    let item = Item {
        id,
        path: canonical.clone(),
        mime: mime.clone(),
        preview: PreviewState::Pending,
        selected: true,
    };
    state.borrow_mut().items.push(item);

    // 6. create cell and append to grid
    let cell = ItemCell::new(id, &canonical, state.clone(), cell_map.clone());
    layout::append_item(flow_box, cell.widget());
    cell_map.borrow_mut().insert(id, cell);

    // 7. spawn preview task
    let path_clone = canonical.clone();
    let mime_clone = mime.clone();
    std::thread::spawn(move || {
        let result = preview::generate(&path_clone, &mime_clone);
        preview_tx.send_blocking(PreviewResult { id, payload: result }).ok();
    });
}
