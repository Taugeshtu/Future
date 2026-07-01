use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use gtk4::gio;
use gtk4::prelude::*;

static ASSOCIATIONS: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn init() {
    let mut map = HashMap::new();
    let config_path = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".config")
        })
        .join("purse/associations.conf");
    if let Ok(content) = std::fs::read_to_string(config_path) {
        for line in content
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
        {
            if let Some((k, v)) = line.split_once('=') {
                map.insert(k.trim().to_string(), v.trim().to_string());
            }
        }
    }
    let _ = ASSOCIATIONS.set(map);
}

const SELF_DESKTOP_ID: &str = "purse-niri.desktop";

pub fn open_files_bypass_self(targets: &[(PathBuf, Option<u32>, Option<u32>)]) {
    for (path, line, col) in targets {
        open_file_bypass_self(path, *line, *col);
    }
}

fn open_file_bypass_self(path: &PathBuf, line: Option<u32>, col: Option<u32>) {
    if let Some(l) = line {
        let editor_cmd = std::env::var("GLUEK_UP_EDITOR_CMD")
            .unwrap_or_else(|_| "lite-xl".to_string());
        let arg = match col {
            Some(c) => format!("{}:{}:{}", path.display(), l, c),
            None => format!("{}:{}", path.display(), l),
        };
        let _ = std::process::Command::new(editor_cmd).arg(arg).spawn();
        return;
    }
    // Pass real file bytes so GIO doesn't mistake the file for application/x-zerosize.
    // &[] with length 0 is treated by GLib as "data present but empty" → wrong type.
    let sniff_data: Vec<u8> = {
        use std::io::Read;
        let mut buf = [0u8; 512];
        std::fs::File::open(path)
            .and_then(|mut f| f.read(&mut buf).map(|n| buf[..n].to_vec()))
            .unwrap_or_default()
    };
    let (content_type, _) = gio::content_type_guess(Some(path), &sniff_data);

    let mut handler = None;
    if let Some(app_id) = ASSOCIATIONS
        .get()
        .and_then(|m| m.get(content_type.as_str()))
    {
        handler = gio::DesktopAppInfo::new(app_id).map(|a| a.upcast::<gio::AppInfo>());
    }
    let handler = handler.or_else(|| {
        gio::AppInfo::all_for_type(&content_type)
            .into_iter()
            .find(|app| {
                app.id()
                    .map(|id| id.as_str() != SELF_DESKTOP_ID)
                    .unwrap_or(true)
            })
    });

    if let Some(app) = handler {
        let file = gio::File::for_path(path);
        if let Err(e) = app.launch(&[file], gio::AppLaunchContext::NONE) {
            eprintln!("purse: failed to open {:?}: {}", path, e);
        }
    } else {
        // No other handler known — fall back to xdg-open as last resort
        let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    }
}
