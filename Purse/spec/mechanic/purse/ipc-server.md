# IPC Server

`ipc.rs` — binds the Unix socket, listens for incoming file paths, feeds ingestion.

## Socket path

```rust
fn socket_path() -> PathBuf {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| format!("/run/user/{}", nix::unistd::getuid()));
    PathBuf::from(runtime_dir).join(format!("purse-{}.sock", std::process::id()))
}
```

## Startup

Called once from `main.rs` before the GLib loop starts:

```rust
pub fn spawn_server(tx: glib::Sender<PathBuf>) {
    let path = socket_path();
    // remove stale socket file if it exists (shouldn't, but be defensive)
    let _ = std::fs::remove_file(&path);
    let listener = std::os::unix::net::UnixListener::bind(&path)
        .expect("failed to bind IPC socket");
    std::thread::spawn(move || run_server(listener, tx));
}
```

## Server loop

```rust
fn run_server(listener: UnixListener, tx: glib::Sender<PathBuf>) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, tx.clone()),
            Err(_) => break,   // listener closed or error; exit thread
        }
    }
}

fn handle_connection(stream: UnixStream, tx: glib::Sender<PathBuf>) {
    // each connection is handled inline (not spawned) — connections are short-lived
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        if let Ok(line) = line {
            let path = PathBuf::from(line.trim());
            tx.send(path).ok();
        }
    }
}
```

One connection at a time is fine: `purse-niri` connects, writes all paths, closes.
The connection is brief. No concurrent connection handling needed in v1.

## Shutdown / cleanup

Register a handler at exit to remove the socket file:

```rust
// in main.rs, after application.run() returns:
let _ = std::fs::remove_file(socket_path());
```

GTK `Application` handles SIGTERM gracefully via GLib; the code after `run()`
executes on normal exit. Abrupt kill will leave a stale socket file — the
`remove_file` at startup handles that case.
