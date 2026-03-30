# Views

> A renderer for content at an address.

A view takes a **content address** (file path, URL, chunk address like `file.md:45-200`) and presents it. It can also edit — emitting patches back to the address. That is the entire interface.

Two views pointing at the same address are two windows into the same content.

## Rules

- No overlapping other views.
- No overlapping the floor (Kelp).
- Free arrangement within the workspace.
- Views do not manage tabs, splits, file trees, or sidebars. Those are the [[strip]]'s job and the [[navigator]]'s job.

## Congruency marks

When a view's origin diverges from the [[strip]]'s current context, it gains a visible **congruency mark**: its origin address. The mark is a navigation target — clicking it navigates the floor there.

Out-of-congruency views are normal and useful. The marks keep you oriented.

## Serialisation

Views exist on a spectrum of restorability:

- **Native views** (implement the view contract): content address + parameters → restore exactly.
- **Legacy apps**: position and file restored; internal app state lost. Better than nothing.

Workspace persistence = serialise every view's address and parameters, restore on reopen.

## The view contract

A native view:

- Initialises with a content address and optional parameters.
- Reports preferred size; negotiates actual size with the compositor.
- Receives notifications when its content address changes externally.
- Can emit patches back to its content address.

See [[The Shape of Work#Views]] and [[The Shape of Work#The protocol]] for the full model.

## Inspiration: FreeCAD annotation

Wish: annotate a sketch right there in FreeCAD — inline text, notes attached to geometry, without leaving the tool.

Extrapolation: imagine if different FreeCAD workbenches and objects were "discombobulated" — each as a separate file, a separate view/app, interspersed with text and annotations. The sketch as a view. The simulation as a view. The BOM as a view. Notes living alongside them, in the same spatial arrangement. This is exactly the kind of thing the view model should make natural.

## Legacy views

Legacy Wayland clients appear as views subject to the same spatial rules. The [[strip]] infers their content address as best it can. Congruency marks are approximate. Serialisation is partial. This is fine — legacy apps run on day one.
