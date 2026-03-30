# Rendering

## Grid layout

Items displayed in a wrapping grid. Each cell: preview + filename label beneath.
Cell size is fixed. Grid wraps to fill window width.

Window width is fixed at launch (reasonable default, maybe user-resizable later).
Height grows with content up to a max, then clips (v1: no scroll — see overview).

## Preview per MIME type

| Type | Preview |
|------|---------|
| `text/*` | First ~20 lines in a `GtkSourceView` buffer (read-only). Language detected from path/MIME on the main thread. |
| everything else | Thumbnail via FreeDesktop thumbnail cache (`~/.cache/thumbnails/large/`). Requested from system thumbnailer (tumbler) over D-Bus if not cached. Falls back to file icon on failure. |

This means any format with a registered thumbnailer works automatically: images,
PDFs, STLs, videos, etc. No per-format code in purse.

## Preview lifecycle in the widget

Preview generation is async. While pending, the cell shows a spinner or placeholder.
On completion (or failure), the cell updates in place.

Failed previews fall back to icon + MIME, not an error state. Silent degradation.

## Window and item borders

The window background is transparent (compositor shows through). To make the
purse bounds legible, the inner container (`.purse-root`) carries a visible border.
Border style is configurable via CSS — the reference style is a solid colour border
with slight border-radius.

Item cells have a border that is transparent at rest and becomes visible on hover
(`.purse-hovered`). Border width should be non-trivial (≥8px) so hover feedback
is immediately obvious. The border is on the inside of the cell, not a separate
highlight element.

Both borders are pure CSS — no layout code involved.

## Selection state in the UI

Selected items: normal appearance.
Deselected items: visually dimmed (reduced opacity or desaturated).

No separate "selected" indicator needed — dimming the deselected is enough.
The default (all selected) means the initial state looks clean, no clutter.
