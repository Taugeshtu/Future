# State

Purse's state is a single ordered list of items, plus a socket handle.

## Item list

Ordered by arrival time (first in, first shown). Each item carries:
- canonical path (the identity key)
- MIME type
- preview payload (pending → ready → failed)
- selected: bool — `false` means the item has been dismissed (single-click)

## Deduplication

Canonical path is the identity key. Ingestion checks before inserting.
Duplicate path → silently skip. No reordering of existing items.

This means: if you send the same file twice (e.g. from two Thunar selections),
it appears once, in the position it first arrived.

## Dismissal (single-click)

Purse is too transient for a persist-and-deselect model. Single-clicking an item
removes it from the visible list immediately: the widget is detached, the cell_map
entry is dropped, and `item.selected` is set to `false`.

`state.items` is a Vec indexed by `ItemId = usize`; entries are never physically
removed (that would invalidate subsequent IDs). Dismissed items sit as tombstones
with `selected = false`. Enter dispatch skips them via `.filter(|i| i.selected)`.

## Hover

The currently hovered item is transient UI state — it lives in Purse state, not in
the widget layer. A single `Option<ItemId>` tracking which item the pointer is over.

Updated by mouse-enter / mouse-leave events on grid cells. Used by `Ctrl+C` to
know which item's content to copy. Visual hover effect (sky-blue border) reads from
the same field via CSS class toggling on the cell root widget.

## What state is NOT responsible for

- Rendering (state holds data, not widgets)
- IPC (socket handle is owned elsewhere; state just receives the results)
- Persistence (state is in-memory, lives with the process)
