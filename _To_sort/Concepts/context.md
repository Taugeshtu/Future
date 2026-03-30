# Context

> The implicit has a name now.

Work has always had context — what you're working with, where, and where your attention lies within it. Previously that was scattered and unstated: the focused window, the file manager's current directory, the editor's cursor position. Each tool held a fragment. Nothing held the whole picture.

Context in Future is explicit, named, and queryable.

## Three primitives

### Location

Where work is happening. Usually a directory. May be a list when multiple locations are in play — ordered, primary first. The floor is always the filesystem; home is the minimum fallback.

Location is the coarsest grain: even when nothing else is known, a location can always be stated.

### Items

What is being worked with. Files, folders, content addresses — the nouns currently in play. May be a single item, a selection, or the contents of a [[Purse]]. Empty is valid.

Items are what verbs act on. The [[launcher]] inherits them. Step one of the noun/verb model is assembling Items.

### Attention

Where within Items focus currently lies. For a text file: a caret (file, line, column). For a list: the highlighted entry. Attention is optional — not every context has a point of attention, and that is fine.

Attention is the finest grain. It enables operations that need to know not just *what* but *where within it*: LSP queries, reference resolution, annotation.