# Future — Components

Future is not an app - it's an idea for how to do computing. Each instance of the Future is an attempt at realizing that idea.

In Linux, Future can be realized using components:

![[navigator]]

---

![[purse]]

---

![[launcher]]

---

## [[text-edi-viewer]]

> Read and edit a single file, with full fidelity.

One window, one file. Syntax highlighting, multiple cursors, find/replace, markdown preview. LSP connection for definition peek (which opens a new window — another lifted view). Explicitly no tabs, no splits, no file tree, no project management. The editor is a verb; the Work Manager handles everything else.

*Buildable now, on niri.*

---

## [[kelp]]
The sleeker, more aligned iteration on niri.

A Wayland compositor built around the strip/navigator/lifted-zone model. Where the other [[components]] become first-class primitives rather than approximations bolted onto an existing compositor. The protocol is the thing that travels if others find value.

*Long-term. The destination, not the starting point.*

---

## Kelp-internal primitives

These live inside Kelp and are not separately buildable (unless?..):

- **[[strip]]** — workspace unit; identity = a folder
- **[[views]]** — artifacts lifted from the floor; content-addressed
- **[[scrollmap]]** — left-edge chunk navigator for tall content
