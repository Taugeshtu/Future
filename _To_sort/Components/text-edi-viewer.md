# Text Edi-viewer

> Read and edit this file. Nothing else.

A thin, single-file view for reading and editing text. Viewing and editing are modes of the same surface. The [[scrollmap]] is mounted on its left edge.

## What it does

- Renders one file at a content address.
- Chunk navigation via the [[scrollmap]].
- Multiple cursors, syntax awareness, inline git blame.
- Mouse selection snaps to word boundaries by default; `Shift+arrows` for character-level precision.

## What it does not do

No tabs. No split panes. No file tree. No project management. No sidebar.

The [[strip]] and [[navigator]] handle spatial organisation. The [[launcher]] handles opening files. The edi-viewer does exactly one thing. Scope creep is a design failure here.

## Chunk granularity for AI

The current chunk (the [[scrollmap]]'s viewport) is the natural context boundary for AI operations via the [[launcher]]. "Explain this" or "fix this" operates on what is visible — the visible chunk is the declared noun. Need more context? Lift more into the [[Purse]].

## Current state

Lite-XL fills this role. With plugins we've stripped the assumption of a project folder — just a lightweight editor with good ergonomics.
