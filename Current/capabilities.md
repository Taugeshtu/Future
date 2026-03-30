# Current — Capabilities

> At the moment of need, the workspace knows what it's doing.

Current resolves [[context]] from the live system at the moment of invocation. It queries whatever is relevant — the focused application, the file manager, a running Purse — assembles the answer, and returns it. One invocation, one answer.

## What it surfaces

Current surfaces the three context primitives defined in [[context]]:

**Location** — the directory where work is happening. Always available; home is the floor.

**Items** — the files or folders currently in play. A single open file is a list of one. A Purse's contents is a longer list. If you want one item from a multi-item context, that choice is yours to make.

**Attention** — the caret: which file, which line, which column. Only available when the focused application can provide it.

## The CLI

Current is invoked on demand — by a hotkey, a script, a verb pipeline. It takes a single argument: the level of context requested.

```
current location   → a directory path
current items      → a list of paths, one per line
current attention  → a file path with line and column
```

The response is either the thing requested, or a failure with the best available location as a consolation:

```
fail:/path/to/best/known/location
```

The contract is strict: you get exactly what you asked for, or you get `fail`. There are no partial results. A caller that can work with less should ask for less. A caller that receives `fail` can recover by asking again at a coarser level — but that is the caller's decision, not Current's.

## Boundaries

Current does not interpret context. It does not know what verbs exist, which Items are relevant, or what Attention means for the task at hand. It answers one question — *what is the context right now?* — and stops there.

Current does not maintain state. It is not a daemon. The workspace is already the state; Current is a lens over it.

## v1 scope

v1 provides Location, Items, and Attention via CLI. Providers are hardcoded for the applications in current use. The socket interface described in [[current.md]] is deferred — Current merges into the Work Manager (eventually [[Kelp]]) before that interface becomes necessary.
