use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const FLAVOR: &str = "large";
const POLL_INTERVAL: Duration = Duration::from_millis(100);
const TIMEOUT: Duration = Duration::from_secs(30);

/// Return the cached thumbnail path if it already exists.
pub fn cached(path: &Path) -> Option<PathBuf> {
    let thumb = cache_path(path);
    thumb.exists().then_some(thumb)
}

/// Ask the D-Bus thumbnailer to generate a thumbnail and wait for it to appear.
/// Returns the thumbnail path on success.
pub fn request(path: &Path, mime: &str) -> Option<PathBuf> {
    request_inner(path, mime).ok()
}

fn request_inner(path: &Path, mime: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let uri = format!("file://{}", path.display());
    let thumb = cache_path(path);

    let conn = zbus::blocking::Connection::session()?;
    let proxy = zbus::blocking::Proxy::new(
        &conn,
        "org.freedesktop.thumbnails.Thumbnailer1",
        "/org/freedesktop/thumbnails/Thumbnailer1",
        "org.freedesktop.thumbnails.Thumbnailer1",
    )?;

    let uris = vec![uri];
    let mimes = vec![mime.to_string()];
    let queue_result: zbus::Result<(u32,)> =
        proxy.call("Queue", &(uris, mimes, FLAVOR, "foreground", 0u32));
    if let Err(e) = queue_result {
        return Err(e.into());
    }

    let deadline = Instant::now() + TIMEOUT;
    while Instant::now() < deadline {
        if thumb.exists() {
            return Ok(thumb);
        }
        std::thread::sleep(POLL_INTERVAL);
    }

    Err("thumbnail generation timed out".into())
}

fn cache_path(path: &Path) -> PathBuf {
    let uri = format!("file://{}", path.display());
    let hash = format!("{:x}", md5::compute(uri.as_bytes()));
    cache_dir().join(format!("{hash}.png"))
}

fn cache_dir() -> PathBuf {
    let base = std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".cache")
        });
    base.join("thumbnails").join(FLAVOR)
}
