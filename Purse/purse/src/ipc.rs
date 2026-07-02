use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

use async_channel::Sender;

pub fn socket_path() -> PathBuf {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| format!("/run/user/{}", libc_getuid()));
    PathBuf::from(runtime_dir).join(format!("purse-{}.sock", std::process::id()))
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct TransientPayload {
    pub uuid: String,
    pub label: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum IpcMessage {
    File(PathBuf),
    Transient(TransientPayload),
}

pub fn spawn_server(tx: Sender<IpcMessage>) {
    let path = socket_path();
    let _ = std::fs::remove_file(&path);
    let listener = UnixListener::bind(&path).expect("failed to bind IPC socket");
    std::thread::spawn(move || run_server(listener, tx));
}

fn run_server(listener: UnixListener, tx: Sender<IpcMessage>) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, tx.clone()),
            Err(_) => break,
        }
    }
}

fn handle_connection(stream: UnixStream, tx: Sender<IpcMessage>) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed = line.trim();
            if trimmed.starts_with('{') && trimmed.ends_with('}') {
                if let Ok(payload) = serde_json::from_str::<TransientPayload>(trimmed) {
                    tx.send_blocking(IpcMessage::Transient(payload)).ok();
                    continue;
                }
            }
            let path = PathBuf::from(trimmed);
            tx.send_blocking(IpcMessage::File(path)).ok();
        }
    }
}

// minimal uid lookup without pulling in the nix crate
fn libc_getuid() -> u32 {
    extern "C" {
        fn getuid() -> u32;
    }
    unsafe { getuid() }
}
