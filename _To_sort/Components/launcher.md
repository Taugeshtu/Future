# The Launcher

> Enact upon the things you are attending to.

The verb selector. Invocable anywhere, over anything. Inherits the current context ([[Purse]] contents, floor selection) as nouns. The second half of the noun/verb model.

The terminal, the file picker, the application launcher, the browser address bar, and the search box are all projections of one interaction: resolve the thing at an address.

## Noun-first

You select nouns first — lift items into the [[Purse]], select files on the [[navigator]] floor, focus a chunk in the [[text-edi-viewer]]. Then invoke the launcher. You know what you're working on before you know which tool to reach for.

## Dispatch

Type into the launcher:

- **URL** → browser view.
- **Shell command** → execute, spawn a view of the output.
- **Search query** → results view.
- **`>`** → routing mode (see below).

## The `>` pipe syntax

`>` invokes routing mode — a visual pipe operator.

- **Type text, then `>`** → destination list appears (bookmarks, recent folders, AI, other views). Text is routed to the chosen destination.
- **Select files, invoke launcher, then `>`** → same destinations, payload is the selected files.

One interface handles: capturing a thought, routing text to a document, moving files, sending content to an AI session.

## Verb surface

Verbs are typed by nouns. The launcher surfaces compatible operations based on what's selected:

- STL file → view, open in slicer, send to printer.
- Text chunk → edit, summarise, translate, pipe to agent.
- Folder → open in new [[strip]], archive, set as destination.

Incompatible verbs are not shown. Discovery is through the nouns you hold.

## Calculator mode

When input looks like a math expression (numbers + operations, parseable as math), the launcher switches to expression evaluation. `Esc` = dismiss. `Ctrl+C` = copy result. `Enter` = pop out as a persistent view.

## The launcher is not an app launcher

A traditional launcher (rofi, fuzzel) is verb-first. This launcher supersedes it — you are invoking a verb against a set of nouns, not "launching an app." The distinction matters for how context flows and how verbs are discovered.

---

## Open design notes

**No-context mode.** Sometimes you don't have context in hand. The launcher still opens as an input box with:
- A pinned area above — grid of pinned apps/folders (a la Windows 8 start menu), manually curated. Possibly auto-populated second row (recently-worked-in folders).
- Arrow navigation on the grid when input is empty; on suggestions when typing.
- Grid dissolves when input is entered; suggestions fade in under (or above) the bar.

**Grid editor.** The launcher UI could be a built-in visual grid editor. Input bar placed within the grid, anchor point configurable. Grid + input + suggestions as one unified surface.

**Hotkey.** Single `Super` press-release (no other keys held). Slow press shouldn't count. May need keyd or similar.

**Input shortcuts:**
- `!command` or `1command` → straight to terminal.
- `/url` → straight to browser address bar (browser autocomplete in-launcher would be ideal but may not be feasible).

**Folder pinning.** Best case: pin folders in the file manager, launcher reads from there.

**Note-taker mode:**
- Snap a screenshot (and later: grab the active URL or file path) when making a note — attach it as a "source" of the thinking. Passive context capture.

