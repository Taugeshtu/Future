use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

use async_channel::Sender;

pub fn socket_path() -> PathBuf {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| format!("/run/user/{}", libc_getuid()));
    PathBuf::from(runtime_dir).join(format!("purse-{}.sock", std::process::id()))
}

pub fn spawn_server(tx: Sender<PathBuf>) {
    let path = socket_path();
    let _ = std::fs::remove_file(&path);
    let listener = UnixListener::bind(&path).expect("failed to bind IPC socket");
    std::thread::spawn(move || run_server(listener, tx));
}

fn run_server(listener: UnixListener, tx: Sender<PathBuf>) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, tx.clone()),
            Err(_) => break,
        }
    }
}

fn handle_connection(stream: UnixStream, tx: Sender<PathBuf>) {
    let reader = BufReader::new(stream);
    for line in reader.lines() {
        if let Ok(line) = line {
            let path = PathBuf::from(line.trim());
            tx.send_blocking(path).ok();
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
