# Binaries

Purse ships as two binaries with distinct responsibilities.

## `purse`

The window. Renders the grid, manages selection, handles IPC and drag-and-drop.

Knows nothing about workspaces, compositors, or whether another instance exists.
That is explicitly not its problem.

On startup it binds a Unix socket unconditionally (`purse-{pid}.sock`).
Anyone who wants to add files to a running instance writes to that socket.

Portable: works in any environment that can run a GTK4 window.

## `purse-niri`

The smart invoker for the Niri compositor environment.

Knows how to:
1. Query Niri IPC for the currently focused workspace
2. Find an existing `purse` window on that workspace (by app-id + workspace membership → PID)
3. Forward files to its socket — or spawn `purse` if none exists

This is what Thunar custom actions call. This is the Niri-specific glue.

When the compositor changes, write `purse-sway` or `purse-hyprland`.
`purse` itself never changes.

## Why split

Singleton-per-workspace behavior is an environment concern, not a Purse concern.
Keeping it out of `purse` means the window binary stays simple, testable, and reusable.
The adapter is thin and entirely replaceable.
