use std::path::PathBuf;

use crate::preview::PreviewPayload;

pub type ItemId = usize;

pub enum PreviewState {
    Pending,
    Ready(PreviewPayload),
    Failed,
}

#[derive(Clone, Debug)]
pub enum ItemKind {
    File {
        path: PathBuf,
        mime: String,
        line: Option<u32>,
        col: Option<u32>,
    },
    Transient {
        uuid: String,
        label: String,
        content: String,
    },
}

pub struct Item {
    pub id: ItemId,
    pub preview: PreviewState,
    pub selected: bool,
    pub kind: ItemKind,
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
