# Purse

> Hold and inspect the things to attend to.


A floating window that accumulates selected files as a persistent context object. Shows you enough of each file to identify and assess it — not full fidelity, but enough. Stays with you as you work. Multiple Purses = multiple independent contexts.

The first half of the #noun/verb model: context declaration made visible and persistent. You assemble context in Purse; you act on it via the [[launcher]].

The name: a purse you take with you. Also: the purse you carry for pearl-diving.

---

## The fidelity escalation

Files exist at multiple levels of examination. [[Purse]] occupies the middle:

```
- filename
- filename + icon / thumbnail
- Purse (enough to assess)
- full viewer / editor
```

For text: first N lines, syntax-highlighted if cheap, plain if not. For images: thumbnail. For PDFs: first page thumbnail. For everything else: icon + metadata, or thumbnailer service.

Double-click on an item in Purse → escalate to full viewer. The content address is already known; the viewer opens immediately.

[[Purse]] can also initiate #verb picking: if no viewer exists for a type, double-click opens the [[launcher]] with that file as the selected noun.

---

## How Purse gets filled

- **Thunar custom action**: select files, trigger action → spawns Purse (or adds to existing Purse on the same workspace) with those files.
- **Drag and drop**: drop files onto an open Purse window.
- **Eventually**: gesture from [[navigator]] floor ("zoom in" on a folder lifts its most-recent files into a new Purse).

---

## Selection and dispatch

Everything in Purse starts selected. Selection = what will be dispatched.

- Click an item to toggle it out of selection.
- Rect-select to redefine selection.
- `Enter` → dispatch all selected items: one full viewer window per file.
- `Ctrl+C` on a focused item → copy its content to clipboard.

The selection state in Purse is the declared context. When the [[launcher]] is eventually invoked, it can read the active Purse's selection as its noun list.

---

## Configuration

Purse reads explicit file associations from `~/.config/purse/associations.conf` (or `$XDG_CONFIG_HOME/purse/associations.conf`).

Format is simple key-value:
```ini
image/png = imv-dir.desktop
image/jpeg = imv-dir.desktop
text/plain = org.lite_xl.lite_xl.desktop
```

If a MIME type is matched, Purse launches the specified desktop application. If not matched, it falls back to GIO/GLib's recommended application list, or `xdg-open` as a last resort.

---

## Multiple Purses = multiple contexts

Multiple Purse windows can coexist on the same workspace. Each is an independent context bundle.

Which Purse receives a new Thunar selection: the most recently focused one on the current workspace. If none exists, a new one spawns. (Multiple Purses on the same workspace: future-problem. For now, assume one per workspace.)

---

## Floating in niri

Purse is a regular floating window in niri. Floating windows in niri travel with you across the strip's column scroll — they stay put while you navigate. This gives Purse the "context that follows you" property without needing layer-shell.

Purse can be moved into the regular tiling layout if you want it anchored. It can also be moved to a different workspace deliberately.

---

## What Purse is not

- Not a file manager. It does not navigate. Navigation is the [[navigator]]'s job.
- Not a full viewer. Purse shows enough to decide on the action, not enough to do the action.
- Not a launcher. It holds nouns; the [[launcher]] handles verbs.
- Not persistent across sessions (for now). Purse is transient working memory.

---

## v1 scope

- Thunar custom action populates Purse with selected files.
- Grid of item previews: text = first ~20 lines, images = thumbnail, unknown = icon.
- Click to deselect, Enter to open all selected in editor (one window per file).
- Ctrl+C copies focused item's content.
- No scroll for large selections (v1). "And N more files" indicator if overflow.
- One Purse per workspace. Multiple instances: later.

## way into the future

Purse is a concept. It can be a separate application, but it can also be part of Kelp (probably not, if we nail the protocol and the system for composability). Whatever is _doing the purse_, it can parse the files, find the wikilinks, and draw them. And even hint at the links that point to things outside of currently lifted set of items (web pages, folders, other files..). These links can be navigatable. This is "the whole system facilitates a web-style navigation". Not one app. A whole system. And apps participate.