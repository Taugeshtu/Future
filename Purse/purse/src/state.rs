use std::path::PathBuf;

use crate::preview::PreviewPayload;

pub type ItemId = usize;

pub enum PreviewState {
    Pending,
    Ready(PreviewPayload),
    Failed,
}

pub struct Item {
    pub id: ItemId,
    pub path: PathBuf,
    pub mime: String,
    pub preview: PreviewState,
    pub selected: bool,
    pub line: Option<u32>,
    pub col: Option<u32>,
}

pub struct PurseState {
    pub items: Vec<Item>,
    pub hover: Option<ItemId>,
}

impl PurseState {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            hover: None,
        }
    }
}

pub struct PreviewResult {
    pub id: ItemId,
    pub payload: Result<PreviewPayload, ()>,
}
