# File Forwarding

`forward.rs` — sends file paths to a running Purse instance over its IPC socket.

## Implementation

```rust
pub fn forward(pid: u32, paths: &[PathBuf]) -> anyhow::Result<()> {
    let socket_path = purse_socket_path(pid);
    let mut stream = std::os::unix::net::UnixStream::connect(&socket_path)
        .context("failed to connect to purse socket")?;

    for path in paths {
        writeln!(stream, "{}", path.display())
            .context("failed to write path to socket")?;
    }

    stream.flush()?;
    // close on drop
    Ok(())
}
```

One connection, all paths written, connection closed. Purse's IPC server reads
them line by line and ingests each. No acknowledgement expected.

## main.rs (purse-niri)

```rust
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
```

Forward failure falls back to spawn. `purse-niri` exits immediately after
either path — it never stays resident.

## `purse` must be on PATH

`spawn()` assumes `purse` is findable via `PATH`. This is a deployment concern:
both binaries should be installed to the same directory (e.g. `~/.local/bin/`).
An alternative is to resolve the path relative to the `purse-niri` binary location
at compile time or runtime — that's an installation detail, not an architecture one.
