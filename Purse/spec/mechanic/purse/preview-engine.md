# Preview Engine

`preview.rs` — generates a `PreviewPayload` for a given path + MIME type.
Runs entirely in a worker thread; no GTK calls here.

## Entry point

```rust
pub fn generate(path: &Path, mime: &str) -> Result<PreviewPayload, ()> {
    if mime.starts_with("text/") {
        generate_text(path)
    } else {
        generate_thumbnail(path, mime)   // everything non-text goes via thumbnailer
    }
}
```

## Text

```rust
fn generate_text(path: &Path, mime: &str) -> Result<PreviewPayload, ()> {
    let file = File::open(path).map_err(|_| ())?;
    let content: String = BufReader::new(file)
        .lines()
        .take(20)
        .filter_map(|l| l.ok())
        .collect::<Vec<_>>()
        .join("\n");

    let language_id = detect_language(path, mime);
    Ok(PreviewPayload::Text { content, language_id })
}
```

Language detection: ask `sourceview5::LanguageManager` by path + MIME.
This requires GLib to be initialized — `LanguageManager::default()` is safe
to call from threads after GTK init. If uncertain, test this; fallback is `None`.

Alternatively detect purely from MIME string + extension without GtkSourceView,
and let the widget do the LanguageManager call on the main thread. Safer.
**Decision: detect in widget (main thread), not here. Pass `None` for language_id
from the engine; item_cell calls LanguageManager when building the SourceView.**

## Thumbnail (images, PDFs, and everything else)

All non-text files go through the FreeDesktop thumbnail system via `thumbnailer.rs`.
This means any format with a registered thumbnailer works automatically — images,
PDFs, STLs, videos, etc. — without purse needing to know about each format.

```rust
fn generate_thumbnail(path: &Path, mime: &str) -> Result<PreviewPayload, ()> {
    if let Some(thumb) = thumbnailer::cached(path) {
        return Ok(PreviewPayload::Image(thumb));
    }
    if let Some(thumb) = thumbnailer::request(path, mime) {
        return Ok(PreviewPayload::Image(thumb));
    }
    Err(())
}
```

`PreviewPayload::Image` carries the path to the thumbnail PNG (not the original
file). Thumbnail PNGs are standard sRGB files that load reliably via
`gtk4::Picture::set_file`.

See `thumbnailer.rs` spec for the cache path formula and D-Bus protocol.

### Why not load images directly?

`gtk4::Picture::set_filename` (deprecated) silently fails on some GTK versions.
`gtk4::Picture::set_file` works for JPEG and SVG but fails on RGBA PNG on certain
GDK builds. Routing everything through the thumbnailer sidesteps all format-specific
loading issues and gives us scaling for free.

## Fallback / icon

```rust
fn generate_icon(mime: &str) -> PreviewPayload {
    let icon_name = gio::content_type_get_icon(mime)
        .and_then(|icon| icon.downcast::<gio::ThemedIcon>().ok())
        .and_then(|themed| themed.names().into_iter().next())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "text-x-generic".to_string());
    PreviewPayload::Icon { name: icon_name }
}
```

## Preview cell dimensions

`PREVIEW_WIDTH` and `PREVIEW_HEIGHT` are constants in this module.
Both item_cell.rs and preview.rs reference them — keep them in one place
(either a shared `constants.rs` or define in `preview.rs` and pub-use).
