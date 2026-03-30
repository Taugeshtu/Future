mod forward;
mod niri;
mod resolution;

use std::path::PathBuf;

use niri::NiriState;
use resolution::Resolution;

fn main() {
    let paths: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();

    if paths.is_empty() {
        eprintln!("purse-niri: no files given");
        std::process::exit(1);
    }

    let niri_state = niri::query().unwrap_or_else(|_| NiriState {
        focused_workspace_id: None,
        purse_windows: vec![],
    });

    match resolution::resolve(&niri_state) {
        Resolution::Forward { pid } => {
            if let Err(e) = forward::forward(pid, &paths) {
                eprintln!("purse-niri: forward failed: {e}, spawning new instance");
                spawn(&paths);
            }
        }
        Resolution::Spawn => spawn(&paths),
    }
}

fn spawn(paths: &[PathBuf]) {
    let mut cmd = std::process::Command::new("purse");
    cmd.args(paths);
    cmd.spawn().expect("failed to spawn purse");
    // detach: don't wait on the child
}
