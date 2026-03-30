use std::path::PathBuf;

use crate::niri::NiriState;

pub enum Resolution {
    Forward { pid: u32 },
    Spawn,
}

pub fn resolve(niri: &NiriState) -> Resolution {
    let focused_ws = match niri.focused_workspace_id {
        Some(id) => id,
        None => return Resolution::Spawn,
    };

    let candidate = niri.purse_windows.iter().find(|w| w.workspace_id == focused_ws);

    match candidate {
        None => Resolution::Spawn,
        Some(w) => {
            if socket_is_live(w.pid) {
                Resolution::Forward { pid: w.pid }
            } else {
                Resolution::Spawn
            }
        }
    }
}

pub(crate) fn purse_socket_path(pid: u32) -> PathBuf {
    let runtime_dir =
        std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(runtime_dir).join(format!("purse-{pid}.sock"))
}

fn socket_is_live(pid: u32) -> bool {
    std::os::unix::net::UnixStream::connect(purse_socket_path(pid)).is_ok()
}
