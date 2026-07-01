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

pub fn generate(path: &Path, mime: &str) -> Result<PreviewPayload, ()> {
    if mime.starts_with("text/")
        || mime.ends_with("toml")
        || mime.ends_with("json")
        || mime.ends_with("xml")
        || mime.ends_with("yaml")
    {
        generate_text(path)
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

fn generate_text(path: &Path) -> Result<PreviewPayload, ()> {
    let file = File::open(path).map_err(|_| ())?;
    let content: String = BufReader::new(file)
        .lines()
        .take(20)
        .filter_map(|l| l.ok())
        .collect::<Vec<_>>()
        .join("\n");
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
