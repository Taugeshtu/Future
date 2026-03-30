### Ports and wiring

Every process on the canvas exposes ports. For legacy programs, these are inferred automatically:
- stdin, stdout, stderr → three ports (two output, one input), untyped.
- argv, environment variables → configuration ports.
- Filesystem access, network access → resource ports, visible as nodes feeding into the program's box.

This is Flatpak's sandboxing model made visual. The user can inspect what resources a program has access to and what data flows through them. Sandboxing becomes a comprehension tool, not just a containment layer.

For native applets (programs built for this system), ports are declared with structure hints and optionally with test suites. The workspace can inspect them, offer compatible wiring targets, and verify substitutability.

Wiring: connecting an output port of one program to an input port of another creates a dataflow edge. The workspace renders these as visible connections. A graph of wired programs can be packaged into a meta-program — a new node with its own external ports, crackable open at any time.