use std::path::PathBuf;

use gtk4::gio;
use gtk4::prelude::*;

const SELF_DESKTOP_ID: &str = "purse-niri.desktop";

pub fn open_files_bypass_self(paths: &[PathBuf]) {
    for path in paths {
        open_file_bypass_self(path);
    }
}

fn open_file_bypass_self(path: &PathBuf) {
    let (content_type, _) = gio::content_type_guess(Some(path), &[]);

    let handler = gio::AppInfo::all_for_type(&content_type)
        .into_iter()
        .find(|app| app.id().map(|id| id.as_str() != SELF_DESKTOP_ID).unwrap_or(true));

    if let Some(app) = handler {
        let file = gio::File::for_path(path);
        if let Err(e) = app.launch(&[file], gio::AppLaunchContext::NONE) {
            eprintln!("purse: failed to open {:?}: {}", path, e);
        }
    } else {
        // No other handler known — fall back to xdg-open as last resort
        let _ = std::process::Command::new("xdg-open")
            .arg(path)
            .spawn();
    }
}
