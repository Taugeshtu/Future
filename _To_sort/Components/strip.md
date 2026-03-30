# The Strip

> Top-level container for organizing work.

A strip holds processes (apps), holds granular context (items via [[Purse]] or other means), and facilitates context assembly. It is the workspace you are *in* — the thing that groups what you're working on and feeds that grouping to the [[launcher]].

Strips stack vertically. Each is independent.

## Concrete implementations

### niri (now)

Window management and arrangement. Grouping is implied by spatial proximity — windows on the same horizontal workspace are "in the same strip." No explicit working directory; context assembly happens through the [[Purse]] as a separate surface.

### Kelp (future)

The strip gains a **floor** — a [[navigator]] wallpapered in, surfaced by pushing windows aside. The floor gives the strip a working directory as identity, making context assembly spatial and gestural: navigate the floor, lift items into the [[Purse]], navigate elsewhere, lift more.

Kelp strips are more congruent with the rest of Future because they win presentation coherence — the work model (noun-first, context assembly, verb invocation) is visually cemented rather than implied.

## Layer stack (Kelp)

1. **Floor** — the [[navigator]]. Always present; infrastructure, not a window.
2. **[[Purse]]** — items held for examination and context assembly.
3. **Floating layer** — transient UI: dialogs, tooltips.
4. **Overlay** — [[launcher]], capture input.

## Capture

Quick-input overlay (hotkey). Lands into the current strip's context by default. `>` routes to a chosen destination. See [[launcher#The > pipe syntax]].

## Congruency (Kelp)

When the floor navigates away from a Purse item's origin, that item gains a **congruency mark** — its origin address, doubling as a navigation target. See [[views#congruency-marks]].

Out-of-congruency items are normal. The mark maintains spatial awareness.
