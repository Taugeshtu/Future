# Infrastructure

## Workspace layout

```
Cargo.toml              [workspace] members = ["purse", "purse-niri"], resolver = "2"
purse/
  Cargo.toml
  src/
purse-niri/
  Cargo.toml
  src/
```

No shared library crate for now. If the IPC wire format needs to be shared
(path framing, error types), extract a `purse-ipc/` crate at that point.

## `purse` dependencies

```toml
[dependencies]
gtk4 = { version = "0.9", features = ["v4_12"] }
sourceview5 = "0.9"
glib = "0.20"
gio = "0.20"
async-channel = "2"
zbus = { version = "5", features = ["blocking"] }   # D-Bus thumbnailer requests
md5 = "0.7"                                          # FreeDesktop thumbnail cache path
```

`gdk-pixbuf` and `poppler` are no longer direct dependencies — thumbnailing is
delegated to the system thumbnailer daemon over D-Bus.

GTK version floor: 4.12 (available in current Fedora stable; gives us
`gtk::FlowBox` improvements and `gtk::Picture` for images).

`poppler` crate wraps libpoppler-glib. Requires `libpoppler-glib-dev` at build time.
If unavailable, PDF preview silently degrades to icon fallback — this is acceptable.

## `purse-niri` dependencies

```toml
[dependencies]
niri-ipc = { git = "https://github.com/YaLTeR/niri", ... }   # may not be on crates.io; verify
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

`niri-ipc` provides typed request/response structs for Niri's socket protocol.
If not publishable as a dependency, re-implement the subset we need
(two request types, two response types) using raw `serde_json` — it's small.

## Runtime requirements

- `libgtk-4` ≥ 4.12
- `libgtksourceview-5`
- `libpoppler-glib` (optional; degrades gracefully if absent at runtime via dynamic linking check or feature flag)
- Niri compositor with IPC socket at `$NIRI_SOCKET` (purse-niri only)

## Build

```
cargo build --release -p purse
cargo build --release -p purse-niri
```

Binaries land in `target/release/purse` and `target/release/purse-niri`.
Install both to wherever Thunar custom action expects to find `purse-niri`.
