# Interactions

## Actions (known)

| Input | Effect |
|-------|--------|
| Click item | Remove item from the list |
| `Enter` | Dispatch all selected items |
| `Ctrl+C` | Copy hovered item's content to clipboard |
| `Escape` | Close / dismiss window |

## Interaction model

State is owned by Purse as plain Rust data. GTK is the view layer only — widgets
reflect state, they do not hold it. No GObject properties as state carriers.

All event handlers (clicks, hotkeys, DnD) mutate state, then notify the view to
update. The view is a function of state. This boundary is intentional: if GTK is
ever swapped out, the state and logic travel intact.

## Hotkey handling in GTK4

`gtk::ShortcutController` with declared `gtk::Shortcut` entries — small fixed set
of shortcuts, no need for dynamic key handling. Worth detailing in mechanistic spec.
