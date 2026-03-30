# lsp-broker

## Rationale

LSP servers are not cheap. `rust-analyzer` can take tens of seconds to index a
project; even lighter servers carry startup cost and memory. If five `lite-xl`
windows open five files from the same folder, the naïve approach spawns five
server processes all doing identical work on identical source. That's wasteful and
fragile — and it only gets worse as more tools in the environment (Purse, future
things) start wanting the same semantic information.

One server per `(project, language)` pair is the right unit. The broker enforces
that invariant system-wide, regardless of how many clients ask.

## What it provides

- **Transparent LSP access for any client.** Clients interact with the broker
  exactly as they would with a real LSP server — no protocol changes, no special
  integration required beyond pointing at the broker instead of the real binary.

- **On-demand server lifecycle.** If a server for the requested language and
  project is already running, the broker routes to it. If not, it starts one.
  Servers can be torn down after all clients disconnect and a cooling-off period
  passes.

- **Multiplexed sessions.** Multiple clients share one underlying server process.
  Each client gets its own logical session; the broker handles the per-client
  bookkeeping so the server sees well-formed protocol on one side and each client
  sees well-formed protocol on the other.

- **Language dispatch.** Given a document URI, the broker determines the language
  (by extension or content) and routes to the appropriate server binary. Which
  server handles which language is a simple configuration table.

## Internal mechanics (high-level)

**Project root resolution.** Given a file path, walk up the directory tree until
a `.git` directory is found. That directory is the project root. Git submodules
are a natural extension: a nested `.git` makes a sub-root, and files inside it
belong to that sub-root rather than the outer project — so each submodule can
have its own server instance if needed.

**Transport.** The broker runs as a daemon and listens on a Unix socket. Clients
talk to the broker via a thin shim that bridges stdio (what editors spawn) to
that socket. From the editor's perspective it just spawned a normal LSP server.

**Session multiplexing.** The broker maintains one connection to each real server
and a separate virtual session per client. It handles the `initialize` /
`shutdown` lifecycle at the broker boundary — the real server is initialized once
and never sees spurious shutdowns while other clients are still connected.
Per-client state (open files, capabilities) is tracked by the broker and
translated into the appropriate server-side operations.

**Server registry.** A running table of `(project_root, language_id) →
server_process`. Starting, reusing, and eventually reaping entries in this table
is the broker's core job.
