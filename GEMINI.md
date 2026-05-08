# Future - Work Management

> "Stdout died at the GUI boundary. The gifted child left behind."

Future is an attempt to do the system's job: **Work Management**. It dissolves application boundaries, treating computing as a flow of nouns (Resources) and verbs (Tools).

## Core Philosophy

- **Nouns over Verbs:** The user should focus on the *thing* they are working on, not the *app* they are using.
- **Composability:** GUI outputs should be as pipeable and redirectable as CLI stdout.
- **Context is Sovereign:** The system must always know what the user is working with (`Current`).
- **Protocols, not Systems:** What lasts is the contract (Ports and Contracts), not the implementation.

## Architectural Primitives

- **Resources:** Everything is a resource (Files, Streams, Memory). Accessed via content addresses.
- **Views:** Single-purpose renderers/editors for resources. The workspace arranges them.
- **Ports & Contracts:** Explicit, typed interfaces for programs. A program is a black box with sockets.
- **The Work Manager (Kelp):** The compositor that knows which views exist and how they relate.

## Sub-Projects

- **Kelp (`/Kelp`):** The Smithay-based Wayland compositor. The orchestrator of the workspace.
- **Purse (`/Purse`):** A GTK4-based "context tray". A spatial container for items in play (the "noun" holder).
- **Current (`/Current`):** An on-demand context resolver. Answers "what is the context right now?" (Location, Items, Attention).
- **TO-DAY (`/TO-DAY`):** A GTK4-based overlay for quick logging and task entry.
- **lsp-broker (`/lsp-broker`):** A system-wide service for multiplexing Language Server Protocol sessions.

## Engineering Standards

- **Language:** Primary development in **Rust** (Edition 2021+).
- **UI Stack:** Wayland protocols, Smithay for the compositor, GTK4 for auxiliary tools.
- **Contextual Awareness:** Tools should leverage `Current` to understand user intent without reinventing context extraction.
- **Documentation:** Architecture-first. If it's not in the spec, it's accidental. Reference `_Thesis/` and `_Implementation/` for deep rationale.

## Gemini CLI Mandates

- **Vision Alignment:** Always prioritize the "Work Management" model. If a change reinforces application silos, it is likely wrong.
- **Surgical Execution:** Changes to the compositor (`Kelp`) must be extremely careful; it is the kernel of the UI.
- **Resource Focus:** When proposing new features, think in terms of Resources and Views. How does this piece of data flow? What port does it use?
- **No Over-Indexing:** While `Kelp` is the heart, the "Future" is the whole organism. Balance attention across all components.
