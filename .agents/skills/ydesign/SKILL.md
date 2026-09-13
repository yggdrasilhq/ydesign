---
name: ydesign
description: Navigate, consult or extend a repository's design system through its DESIGN.md doors and the yggui base notebooks — routing UX tasks, reading inheritance, registering projects, and rendering interactive emd notebooks. Use when doing any UX/visual work in a repo with design/ + DESIGN.md, or when scaffolding that layout for a repo that lacks it.
---

# Ydesign

The design system of a repository is files: a root `DESIGN.md` pointer page
(doors, not rooms), a `design/` tree of decision notebooks, inheritance
records, assets, and — where the project has interactive pages —
`design/notebooks/*.emd`. The layout contract is
[docs/design-layout.md](../../../docs/design-layout.md) (version 1.1.0) in
this repository. Nobody needs the ydesign app to work in a repository; only
`design/notebooks/*.emd` need it to render.

**The reader ships nothing.** The BASE design language lives in libyggterm
(`design/notebooks/*.emd` — the component vocabulary); yggterm owns its
chrome books; every app owns its own books. The ydesign app is the reader
and the shelf is assembled from the registry. Onboarding a host means
registering the repos below and letting the skill keep them fresh:

- libyggterm (id `yggui`) — the base chain every project inherits;
- yggterm (id `yggterm`) — the shell chrome books;
- yggui-priv-modules (id `yggui-priv`) — the private modules and their design books;
- each product repo by its own id.

## Entry procedure (every UX task, every repo)

1. **Open the doors.** Read the project's root `DESIGN.md` and pick the door
   that owns your decision — brand in `design/00-brand.md`, components in
   the component notebooks, the interactive pattern in
   `design/notebooks/*.emd`. Never invent a value a door already decides.
2. **Read the chain.** `design/Inheritance.md` names the layer and parents;
   read the complete parent chain into the libyggterm base notebooks before
   overriding anything. Undefined decisions inherit; explicit local
   decisions override; cycles and unresolved overlaps are invalid.
3. **Work at the owning layer.** A missing component grows the lower layer
   (a yggui component with a gallery entry), never a hand-rolled second
   encoding inside one app. App-specific content stays in the app layer.

## Keep the registry and the skill fresh (automatic, per touch)

When this skill is used on a repository, it maintains the organized state —
do this without being asked:

- **Register the project**: ensure the repo is in the ydesign registry at
  `~/.yggterm/config/ydesign/projects.json` (`ydesign init <repo> --id <id>`
  creates only missing scaffolding and registers; use an isolated
  `--config` for worktrees, then register the persistent checkout after
  landing).
- **Refresh the installed skill copy** at `~/.yggterm/skills/ydesign/` from
  the repo's `.agents/skills/ydesign/` so every host loads the current
  version from the organized root.
- **Name the skill door**: ensure the project's `DESIGN.md` carries a
  Navigation skill line pointing at its skill copy, so the next agent finds
  this procedure where it found the design system.

## Authoring decisions

Keep brand identity, palette, typography and visual rationale in notebooks
and assets, with rationale and rendered examples; `DESIGN.md` only routes.
Preserve existing brand decisions when migrating. For a component decision,
identify the human task and common mouse/keyboard path first; show a large,
legible specimen and a meaningful state transition; explain the rejected
pattern; label proposals separately from measured implementation.

Authoring a component chapter (a book page) follows the living-books
contract: one row group per book, the chapter hosts a real interactive
Dioxus mini-app with deterministic demo data and Reset, source and revision
exposed. A still image, callback counter or external link does not satisfy
the contract — see docs/living-design-books.md in the ydesign repository.

## Visual design ownership

Visual design — componentry, mock generation, iconography, visual logic —
belongs to the fleet's designated UX consultant through the consulting
system; workers wire, build, verify and land. Consult before designing:
stage the task, current pixels, interaction paths, hard constraints and the
full state inventory; demand the design in a reviewable shape (mock-first:
mock, then review, then construction). Mock stills are SVG when malleability
matters and PNG only when raster is the point. Never self-design visuals
under constraint — queue the consult and continue wire-only.

## Evidence

Use the yggui-app-control named shadow for desktop evidence. Record build,
viewport, state, target geometry and faithful pixels. Use invented fixture
data in published examples; private captures stay out of public repos. Do
not claim a disabled control is interactive. Proposed designs remain
proposals until their component implementation receives its own pixel and
interaction proof.
