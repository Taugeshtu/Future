# Future

## Why is Future

Knowledge work is comprehension and synthesis: you observe things, sit with them, and output the semantic compression and/or extrapolation so you don't have to re-derive it. The tools of computing should serve that loop.

They don't do it well. The friction of retrieval is real even when storage is infinite. Your mental index has a fixed budget. Search shortens the friction; doesn't eliminate it. And the context you naturally accumulate while working — open files, tabs, recent folders, related notes — is scattered across application silos, held hostage by app boundaries that are accidental, not essential. The work is shaped to be verb-first: open an app, open work item inside an app. Move between apps, not the things you're working on. Want to pass a thing from one app into another? Find the file first.

Those boundaries exist because the GUI killed composability by swallowing stdout. With no work manager to hold context, every sufficiently large application built one internally — tabs, splits, session state, file trees, recently opened lists. The application subsumed what should have been the system's job, because the system wasn't doing it. Current desktops give you blunt workarounds — virtual workspaces, browser profiles, pinned tabs — because users discovered they needed compartments and had to build them out of the wrong materials.

Future is an attempt to do the system's job. Let's call it "Work Management".

## What is Future

Items and tools of work exist on a gradient of affordance — from what's already in hand to what you'd have to go retrieve. To do Work Management, you need to:
- retrieve things for work; both known things and unknown
- organize items and tools of work within your affordance gradient as is appropriate for the work-in-the-moment

To do Work, you further need to:
- examine things of interest (to decide on action)
- take action(s) upon things

To help with this, Future must provide ergonomic containers that users can use to shape their work. Then context is more immediately surfaced: where work is happening, what's in play, and where within it your attention sits - a more noun-oriented model. To compound the noun/verb model, the boundaries of applications should be dissolved.

Future is not a collection of apps. It's not a protocol. It's an idea about how computing should work — expressed through a set of capabilities.

## How to Future

A Future can be built today, on Linux, from existing and invented tools wired together. The working model: niri as compositor, Thunar as primary noun navigator, Purse as context tray, Lite-XL for reading and editing files, and a launcher to close the noun/verb loop. The seams are visible and held together by scripts — that's fine. The seams reveal exactly what to build next.
