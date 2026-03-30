use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize)]
struct NiriWorkspace {
    id: u64,
    is_focused: bool,
}

#[derive(Deserialize)]
struct NiriWindow {
    app_id: Option<String>,
    workspace_id: Option<u64>,
    title: Option<String>,
}

pub struct NiriState {
    pub focused_workspace_id: Option<u64>,
    pub purse_windows: Vec<PurseWindow>,
}

pub struct PurseWindow {
    pub workspace_id: u64,
    pub pid: u32,
}

fn niri_socket_path() -> PathBuf {
    std::env::var("NIRI_SOCKET")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| "/tmp".into());
            PathBuf::from(runtime).join("niri/socket")
        })
}

fn pid_from_title(title: &str) -> Option<u32> {
    title.strip_prefix("purse-")?.parse().ok()
}

pub fn query() -> anyhow::Result<NiriState> {
    let path = niri_socket_path();
    let stream = UnixStream::connect(&path)
        .with_context(|| format!("failed to connect to niri socket: {path:?}"))?;
    let mut write_stream = stream.try_clone()?;
    let mut reader = BufReader::new(stream);

    use std::io::Write;
    writeln!(write_stream, r#"{{"Workspaces": null}}"#)?;
    let workspaces: Vec<NiriWorkspace> = parse_response(&mut reader, "Workspaces")?;

    writeln!(write_stream, r#"{{"Windows": null}}"#)?;
    let windows: Vec<NiriWindow> = parse_response(&mut reader, "Windows")?;

    let focused_workspace_id = workspaces.iter().find(|w| w.is_focused).map(|w| w.id);

    let purse_windows = windows
        .iter()
        .filter(|w| w.app_id.as_deref() == Some("dev.purse"))
        .filter_map(|w| {
            let workspace_id = w.workspace_id?;
            let pid = w.title.as_deref().and_then(pid_from_title)?;
            Some(PurseWindow { workspace_id, pid })
        })
        .collect();

    Ok(NiriState {
        focused_workspace_id,
        purse_windows,
    })
}

fn parse_response<T: for<'de> serde::Deserialize<'de>>(
    reader: &mut impl BufRead,
    key: &str,
) -> anyhow::Result<Vec<T>> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    anyhow::ensure!(!line.is_empty(), "niri closed connection unexpectedly");

    let val: serde_json::Value =
        serde_json::from_str(line.trim()).context("failed to parse niri response")?;

    if let Some(err) = val.get("Err") {
        anyhow::bail!("niri returned error: {err}");
    }

    let items = val
        .get("Ok")
        .and_then(|ok| ok.get(key))
        .ok_or_else(|| anyhow::anyhow!("unexpected niri response shape for '{key}'"))?;

    serde_json::from_value(items.clone())
        .with_context(|| format!("failed to deserialize niri '{key}' list"))
}
