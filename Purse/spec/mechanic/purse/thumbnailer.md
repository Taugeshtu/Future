# Thumbnailer

`thumbnailer.rs` — FreeDesktop thumbnail cache lookup and async generation via D-Bus.

## Cache path formula

```rust
fn cache_path(path: &Path) -> PathBuf {
    let uri = format!("file://{}", path.display());  // must be absolute canonical path
    let hash = format!("{:x}", md5::compute(uri.as_bytes()));
    cache_dir().join(format!("{hash}.png"))
}

fn cache_dir() -> PathBuf {
    let base = env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env::var("HOME").unwrap()).join(".cache"));
    base.join("thumbnails").join(FLAVOR)  // FLAVOR = "large" (256×256)
}
```

The URI is the `file://` URI of the **canonical absolute path** — the same path
produced by `std::fs::canonicalize`. Any deviation (symlink not resolved, trailing
slash, wrong slash count) produces a different hash and a cache miss.

## Flavor

`"large"` (256×256). Thunar typically requests `"normal"` (128×128); using `"large"`
means purse generates higher-quality thumbnails that Thunar won't have pre-cached,
which is intentional — purse is the "more than Thunar" viewer.

## Public interface

```rust
/// Return cached thumbnail path if it exists on disk.
pub fn cached(path: &Path) -> Option<PathBuf>

/// Request generation via D-Bus, poll until the file appears. Returns path on success.
pub fn request(path: &Path, mime: &str) -> Option<PathBuf>
```

## D-Bus protocol

Service:   `org.freedesktop.thumbnails.Thumbnailer1`
Object:    `/org/freedesktop/thumbnails/Thumbnailer1`
Interface: `org.freedesktop.thumbnails.Thumbnailer1`

```rust
// zbus blocking proxy — Proxy::new (not builder — builder doesn't exist in zbus 5.x)
let proxy = zbus::blocking::Proxy::new(&conn, DEST, PATH, IFACE)?;

// Return type must be annotated explicitly — turbofish on .call() mis-infers
let queue_result: zbus::Result<(u32,)> =
    proxy.call("Queue", &(uris, mimes, FLAVOR, "foreground", 0u32));
```

`Queue` args: `(uris: Vec<String>, mimes: Vec<String>, flavor: &str, scheduler: &str, handle_to_dequeue: u32)`
`Queue` returns: `(handle: u32)`

After queuing, poll `cache_path` every 100ms up to 30s. The thumbnail is written
atomically by the thumbnailer daemon (tumbler on most systems), so `exists()` is safe.

## Pitfalls

- **Wrong MIME kills the request silently.** Tumbler looks up the thumbnailer plugin
  by MIME type. If ingestion passes `application/x-zerosize` (what GIO returns when
  given `&[]` content data and the extension isn't in the local mime-db), tumbler has
  no plugin registered for it and drops the request without error or signal.
  Fix: always pass content-sniffed bytes to `content_type_guess` in ingestion.

- **`Proxy::builder` does not exist in zbus 5.x.** Use `Proxy::new` directly.

- **Explicit return type annotation required.** `proxy.call::<_, RetType>()` turbofish
  confuses type inference in zbus 5.x. Use a `let` binding with a type annotation instead.
