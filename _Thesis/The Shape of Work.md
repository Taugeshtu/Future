# The Shape of Work

---

Computing augments cognition. Specifically: memory (record, store, retrieve), analysis (transform, compare, generate), and communication (exchange across time and space). The most powerful way to use a computer is not dialog — it is building up processes for work, often on the fly, by composing small operations on data.

We had composability once. The terminal gave us pipes: small tools, data flowing between them. It was brittle, untyped, and arcane, but the model was right. Then GUI happened, and we left composability behind. Program output stopped being a grabbable, redirectable, pipeable thing. It got swallowed by the application container. **Stdout died at the GUI boundary.**

The file manager gives you verbs for files. The terminal gives you verbs for streams. Nobody gives you verbs for program outputs in GUI-land. There is no protocol for "I produce, you consume, someone watches." So applications became monoliths — each one reinventing window management, session state, layout, and navigation inside its own box. Every sufficiently advanced application contains an ad-hoc, informally-specified, bug-ridden, slow implementation of a workspace manager.

The "application" is an accidental boundary, not an essential one.

---

## Primitives

### Resources

Everything is a resource. Files are resources. Memory regions are resources. Streams are resources. A resource is bytes, a structure hint, and a durability characteristic (persistent, transient, temporal). The interface to all resources is the same: request content at an address, receive bytes and a hint about what they mean, optionally subscribe to changes.

A file path, a URL, a pipe from a process, a query against a database — these are all content addresses. The browser's address bar is a content address resolver, materialised. Detach it from the browser, generalise the resolver, and you have the foundational primitive.

### Structure hints

Bytes are meaningless without knowing what they represent. The structure hint is the answer — the same problem as MIME types, file metadata, content negotiation, and what XML schemas gestured toward (resolvable structure definitions; right idea, fragile execution via location-addressed URLs). Structure descriptors can be schemas, protocol definitions, or embedded in the data itself. What's needed: a stable, content-addressable identifier for a structure schema, discoverable on demand.

### Views

A view is a renderer. It takes a content address and a structure hint and presents the content. It can also be an editor — emitting patches back to the address. That is the entire interface. Views are thin, single-purpose. One file, one view. The workspace arranges them, not the view itself. Tabs, panes, file trees, sidebars — these are the view trying to be a workspace because the actual workspace doesn't do its job.

### Ports

Programs are not launched in a vacuum. They have arguments, environment, filesystem access, network sockets, configuration, library dependencies. All of these are **ports** — they are just not called that, not enumerable, not inspectable, not uniformly addressable. Make ports explicit and structured, and a program becomes a black box with labelled, typed sockets. Its output becomes a content address that other programs or views can bind to.

### Contracts

The set of a program's typed ports — names, structure hints — is its **contract**. Its port surface. Hash that surface: anything with this hash speaks this exact protocol. The implementation behind the hash can change freely. If the surface changes, that is a new hash, a new contract. Semver becomes structural, not promissory. Mechanical substitution becomes possible: any component with the same surface hash is a drop-in replacement.

Adding a port should not break the hash of existing ones. Per-port hashing with concatenation gives additive compatibility — the same principle as protobuf schema evolution.

**Behavioural contracts:** types capture shape, not behaviour. Unit tests close this gap. A test suite is an executable specification of what a port does. Hash the tests alongside the surface: structure for shape, tests for behaviour. Substitutability becomes verifiable.

### The protocol

The protocol between a view and its workspace is small:

- `bind(address) → content stream + structure hint`
- `emit(patch) → address`
- `notify(address changed)`
- `negotiate(size) ↔ workspace`

If the protocol is small, the model is probably right. Complexity belongs in content and relationships, not plumbing.

---

## The workspace

The workspace knows which views exist, where they are, and what relationships hold between them. It is the graph. It routes content to views and patches from views back to content.

It is not a window manager. It is a **work manager**.

Arrangement is free-form by default. Structure — tiling, grouping, stacking — is opt-in, a gesture, not a policy. The workspace has affordances for arrangement, not rules about it.

Program outputs are visible entities in the workspace, with the same affordances as any other content. They can be inspected, piped, saved, wired into other programs. Shell scripting becomes visual pipelines: observable, examinable, less arcane. Packaging a wired graph of nodes into a new node with its own ports is function composition — crackable open, inspectable at any depth.

Workspace state is serialisable: if every view declares its content address and parameters, persisting a workspace is serialising that list, restoring it is relaunching views, and folding it is collapsing it to an entity that can be unfolded later. Session management at the workspace level, not hacked together from app-level state restore and startup scripts.

---

## Namespacing and organisation

Hierarchy is not opposed to tags. A file path is a tag chain: `~/projects/work/thing/docs/TODO.md` is the entity `TODO.md` carrying the ordered tags `[projects, work, thing, docs]`. The underlying data model is a graph — a bag of trees with cross-links within and between them. Hierarchy is one projection. The workspace offers it; it does not enforce it.

Relationships between entities are themselves entities — metadata attached to content is just more content at a different address. Connections, tags, annotations all live at the same level as the things they describe.

---

## Composition and discovery

Dragging an output port queries the workspace: what installed components have an input port matching this structure? The workspace presents compatible destinations. Composition becomes discoverable — the user explores what is possible by seeing what fits.

Every dmenu, every address bar, every Spotlight, every terminal prompt is the same primitive: text in, resolve an address, produce a view of the result. Unify the resolver and they collapse into one interaction.

---

## Adoption

The model works at every scale — within a program, across machines, up and down the abstraction stack. It starts today. Legacy programs participate cheaply: stdin/stdout/stderr are three untyped ports, argv and env are configuration ports. A legacy program's sandbox (filesystem access, network, environment — what Flatpak already models) becomes a visible set of resource nodes. Sandboxing becomes comprehension, not just containment.

The incentive gradient: structured ports are nicer to work with. People will write native components because it is easier, not because it is mandated.

Agentic tools accelerate the transition. LLMs handle mechanical transformations well — wrapping existing functions to expose typed ports, generating test scaffolding from specifications. The architectural decomposition remains human. In tandem with agentic coding, docs-first development reaches equal footing with "just ship it": when an agent can cheaply turn a spec into tests into implementation, writing the spec first is no longer slower.

This does not require universal adoption. It works for one person. Scale is a consequence, not a prerequisite.

---

## Prior art

Each of these systems saw part of the picture clearly.

**Plan 9** — everything is a resource; files are resources. The 9P protocol is roughly the right size (~13 message types) but the wrong shape (locked to file semantics). Migration cost killed adoption: wrapping legacy was more work than using legacy.

**Unix pipes** — composability via small tools and data flow. Untyped byte streams and arcane incantations limited the audience to specialists.

**Flow-Based Programming** (Morrison, 1970s) — programs as black boxes with typed ports, information packets flowing between them. Formally sound, still niche after fifty years. Text-in-a-file remained good enough.

**LabVIEW** — proof that visual dataflow works for non-programmers. Also proof that it becomes spaghetti at scale, and that trapping it inside one application for one domain limits reach.

**The Semantic Web** — URIs for schemas, typed and composable structured data. Technically sound; failed because it required the ocean to boil before delivering local value.

**Perkeep** — content-addressable blobs with claims as the organising principle. No tree. Architecture sound, implementation not complete enough to serve as a base layer.

**Jupyter** — code and documentation intertwined, dynamically bindable inputs, spatial arrangement of compute fragments. Closest pointer at the experience, trapped in a browser and not composable with other tools.

**Unison** — content-addressed functions, hash of type signature as identity. Same principle as port-surface hashing, at the language level rather than the OS level.

**Dynamicland** — computing projected onto physical surfaces, collaborative, spatial, embodied. Proof that the model works when the medium is the room itself.

The pattern: clean model, real insight, sound architecture. Then the messy reality of migration costs, ecosystem economics, and tooling gaps grinds it down. Not to nothing — the ideas keep resurfacing because they are right. The distinguishing factor for a new attempt: work locally first, scale outward; protocols, not systems; build for yourself.

---

## Open questions

**Dataflow execution model.** When does downstream execution happen? Pull/lazy (spreadsheets), push/reactive (signals), or gated/explicit (LabVIEW)? Circular pipelines, cascading updates, dynamic suppliers of handles — unsolved. Most node editors enforce DAGs. This is architectural.

**Metadata boundary.** How much structure lives in the content versus in the workspace graph? "These bytes are markdown" is intrinsic. "This markdown is tagged 'architecture'" is extrinsic. The boundary is a design decision.

**Sequentialisation.** Spreadsheet rows have an order. In a non-hierarchical, UUID-addressed model, how is ordering expressed?

**Integration testing.** Tests-as-contract covers individual nodes. Emergent behaviour from composition can violate contracts without any single node being wrong.

**Text input in spatial contexts.** Keyboards are awkward in XR. Voice is unreliable. Unsolved.

---

## Touchstones

- Stdout died at the GUI boundary. The gifted child left behind.
- Protocols, not systems. What lasts is the contract, not the implementation.
- Build it for yourself. Scale is a consequence, not a prerequisite.
- The computing model should afford composability — not as a feature, but as a property of the medium.
- Closures are poor man's objects, and objects are poor man's closures. The duality confirms the abstraction level is right.