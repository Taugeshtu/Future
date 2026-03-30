# Current

> At the moment of need, the workspace knows what it's doing.

## The problem

The Future dissolves application boundaries. Nouns flow between verbs. But making
that work requires that the system can answer a simple question at any moment:
*what is the user working with right now?*

Today that question gets answered ad-hoc, per-script, per-action. "Open terminal
here" checks the focused window class, pokes thunar for its CWD, launches the
terminal. Every action reinvents the same grubby extraction logic. Adding a new
editor means patching every script that cares.

## What Current is

Current is an on-demand context resolver. When a verb invocation happens — a
hotkey fires, the Launcher opens — something asks Current: *what's the context
right now?* Current queries whatever is relevant (the WM, the focused app, the
file manager if it's open), assembles a context bag, and returns it.

It does not maintain state. The WM already maintains state: open windows and
running apps *are* the workspace context. Current is a lens over that, invoked at
the point of need. Nothing to keep alive, nothing to sync, nothing to drift.

## Two interfaces

**CLI** — simple, surface-level. Given the current context: a file path? a list of
files? a directory? These are broadly useful and lean on *nix conventions that are
already well-understood and reliably available. Scripts and hotkeys use this.

**Socket** — richer queries for specialty use. Caret position. Provenance (which
app provided this context, and how confidently). The kinds of structured
information that would be brittle as CLI flags but are well-formed as a typed
protocol.

## What v1 provides

At the moment of invocation, Current can tell you:

- Which window is focused and what application it is
- The current file (if the focused app has one open)
- The current directory (from the file's location, or from the file manager)
- If the focused thing is a Purse: the list of files it holds

Providers are hardcoded. If thunar is running, ask it this way. If VSCode is
focused, ask it this way. Pragmatic and shippable.

## An aspiration

You are in any text editor — lite-xl, VSCode, something else entirely. Cursor is
on a symbol. You press a hotkey: *what does this refer to?*

Current is queried for the current file and caret position. That pair goes to
lsp-broker, which routes to the appropriate language server. The answer comes back
as an overlay: the definition, the type, the documentation — right there, without
leaving what you were doing.

The magical part: it works in any editor, so long as that editor can give Current
a filename and a cursor position. The language intelligence is not owned by the
editor. It lives at the system level, accessible to everything.

That's what Current enables. Not by being clever, but by being the place where
context lives at the moment it's needed.
