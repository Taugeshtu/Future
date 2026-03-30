# The Scrollmap

> The whole document at ant size. Navigate by pointing at where you want to be.

A vertical minimap on the **left edge** of a [[text-edi-viewer]] (or any tall-content view). Renders the full content at reduced scale. A viewport indicator shows the current chunk; moving it changes what's displayed.

## Directionality

Left = general, right = specific. The scrollmap is the whole; the chunk view to its right is the detail. This mirrors the [[navigator]]'s Miller columns and the system's left-to-right information hierarchy.

## Chunks

A chunk is a view with a ranged content address: `file.md:0-200`. The scrollmap's indicator position maps directly to a chunk address. Navigating the scrollmap = navigating the chunk address space.

Chunks are first-class. Two chunks of the same file can be open side by side — two views with different ranges, both indicators visible on the same scrollmap. Cross-reference becomes spatial.

## Defining chunks by selection

Drag-select a region on the scrollmap → defines a new chunk, pulls it into a new view. The scrollmap is not read-only; it is a spatial index you can reach into.

## Beyond byte ranges

A chunk's "range" can be a query result: a time window of logs, a severity filter, a search result set. The scrollmap represents these too — the filter is part of the content address.
