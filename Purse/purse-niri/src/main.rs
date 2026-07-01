mod niri;
mod resolution;

use niri::NiriState;
use resolution::Resolution;

fn main() {
    let niri_state = niri::query().unwrap_or_else(|_| NiriState {
        focused_workspace_id: None,
        purse_windows: vec![],
    });

    let pid = match resolution::resolve(&niri_state) {
        Resolution::Forward { pid } => pid,
        Resolution::Spawn => spawn(),
    };
    println!("{}", pid);
}

fn spawn() -> u32 {
    use std::io::Read;
    use std::process::Stdio;

    let mut cmd = std::process::Command::new("purse");
    cmd.stdout(Stdio::piped());
    let mut child = cmd.spawn().expect("failed to spawn purse");

    if let Some(mut stdout) = child.stdout.take() {
        let mut buf = [0u8; 5];
        if stdout.read_exact(&mut buf).is_err() {
            eprintln!("purse-niri: child failed to signal readiness");
            std::process::exit(1);
        }
    }
    child.id()
}
