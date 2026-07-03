use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use gtk4::gio;
use gtk4::prelude::*;

use crate::thumbnailer;

pub const PREVIEW_WIDTH: i32 = 240;
pub const PREVIEW_HEIGHT: i32 = 10;

#[derive(Clone)]
pub enum PreviewPayload {
    Text { content: String },
    Image(PathBuf),
    Icon { name: String },
}

pub fn generate(path: &Path, mime: &str, target_line: Option<u32>) -> Result<PreviewPayload, ()> {
    if mime.starts_with("text/")
        || mime.ends_with("toml")
        || mime.ends_with("json")
        || mime.ends_with("json5")
        || mime.ends_with("xml")
        || mime.ends_with("yaml")
        || mime.ends_with("shellscript")
        || mime.ends_with("fishscript")
        || mime.ends_with("javascript")
        || mime.ends_with("typescript")
        || mime.ends_with("desktop")
        || mime.ends_with("sql")
        || mime.ends_with("x509-ca-cert")
        || mime.ends_with("pem-key")
        || mime.ends_with("php")
        || mime.ends_with("ruby")
        || mime.ends_with("perl")
    {
        generate_text(path, target_line)
    } else {
        generate_thumbnail(path, mime)
    }
}

fn generate_thumbnail(path: &Path, mime: &str) -> Result<PreviewPayload, ()> {
    if let Some(thumb) = thumbnailer::cached(path) {
        return Ok(PreviewPayload::Image(thumb));
    }
    if let Some(thumb) = thumbnailer::request(path, mime) {
        return Ok(PreviewPayload::Image(thumb));
    }
    Err(())
}

fn generate_text(path: &Path, target_line: Option<u32>) -> Result<PreviewPayload, ()> {
    let file = File::open(path).map_err(|_| ())?;
    let lines: Vec<String> = BufReader::new(file).lines().filter_map(|l| l.ok()).collect();
    let content = if let Some(target) = target_line {
        let idx = target.saturating_sub(1) as usize;
        let start = idx.saturating_sub(10);
        let end = std::cmp::min(idx + 11, lines.len());
        let mut s = if start > 0 { vec!["...".to_string()] } else { vec![] };
        for (i, l) in lines.iter().enumerate().skip(start).take(end - start) {
            s.push(format!("{}{}", if i == idx { "-> " } else { "   " }, l));
        }
        if end < lines.len() { s.push("...".to_string()); }
        s.join("\n")
    } else {
        lines.into_iter().take(20).collect::<Vec<_>>().join("\n")
    };
    Ok(PreviewPayload::Text { content })
}

fn generate_icon(mime: &str) -> PreviewPayload {
    let icon = gio::content_type_get_icon(mime);
    let icon_name = icon
        .downcast::<gio::ThemedIcon>()
        .ok()
        .and_then(|themed| {
            themed
                .names()
                .into_iter()
                .next()
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "text-x-generic".to_string());
    PreviewPayload::Icon { name: icon_name }
}
