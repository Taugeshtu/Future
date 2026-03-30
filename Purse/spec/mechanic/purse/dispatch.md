# Dispatch

`dispatch.rs` — acts on the selected file list.

## v1: open with default app

```rust
pub fn open_files(paths: &[PathBuf]) {
    for path in paths {
        let uri = format!("file://{}", path.display());
        if let Err(e) = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE) {
            eprintln!("purse: failed to open {}: {}", uri, e);
        }
    }
}
```

One GIO call per file. GIO resolves the default application for each file's
MIME type and launches it. Each file opens in its own application window.

`AppLaunchContext::NONE` is correct for our use: we don't need to pass display
or startup notification context explicitly; GIO infers them from the environment.

## Future: hand off to launcher

When the launcher component exists, dispatch will instead:
1. Collect selected paths into a list
2. Pass the list to the launcher via an agreed protocol (TBD)
3. The launcher presents verb options for that noun set

The function signature `open_files(paths: &[PathBuf])` should remain stable.
The implementation swaps out. Nothing else in purse needs to change.
