<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Spec — the design/ layout: every repository's design system as files

**Version 1.0.0. Owner specification, 2026-09-13.** This is the contract the
ydesign 1.0.0 release publishes. It makes a repository's design system a
filesystem layout, the same way the fleet memory is a filesystem of doors:
UNIX-shaped, readable with no tool, and renderable by ydesign exactly where
the files are interactive.

## Why

Design rules scattered across prose, chat and tribal memory decay the same
way unrecorded decisions always decay. The fix that already worked for
memory — one file per fact, one index line per door — transfers to design:
one repository, one `design/` tree, one pointer page. A repository that
satisfies this layout can be read by a person, an editor, an agent, or the
ydesign app without any of them needing the others.

## The layout (normative)

Every repository that ships user-facing surfaces — every public app and
library, and every private product repository — carries:

```
DESIGN.md                        # the pointer page (see "DESIGN.md" below)
design/
  Inheritance.md                 # layer, parents, order, what is overridden
  00-brand.md … NN-*.md          # decision notebooks (plain markdown)
  notebooks/                     # INTERACTIVE notebooks (*.emd)
  assets/
    icons/                       # the SVG icon blocks this repo uses
    fonts/                       # font files and their licence notes
    components/                  # component-scoped assets (css, svg, fixtures)
    img/                         # illustrations, mock stills, captures
```

`assets/` subdirectories are by kind, not by notebook; a notebook references
an asset with its repo-relative path, and a flat dump is a defect. The
`00-brand.md` numbering is a convention, not a requirement; the requirement
is that decision notebooks are plain markdown files directly under
`design/`.

## DESIGN.md is a pointer page

`DESIGN.md` is modeled on the memory index, not on a document: **doors, not
rooms**. One headed line per door — a notebook, an asset family, the
inheritance chain, the ydesign skill, the project's live preview. The body
of a door lives in the file the line points at.

A `DESIGN.md` that explains a palette in prose has failed: the palette
belongs in `design/00-brand.md` and `DESIGN.md` carries the one line that
routes to it. A reader — human or agent — must be able to answer "where
does this decision live?" from `DESIGN.md` alone, in one screen.

## Nobody needs ydesign

Working in a repository never requires the ydesign app. `DESIGN.md`,
`design/*.md`, `design/Inheritance.md` and `design/assets/` are plain files:
every editor, every grep, every agent reads them directly.

The only files that need ydesign are the **interactive notebooks**,
`design/notebooks/*.emd` — the pages that embed live mini-apps of the
components they teach. ydesign renders and shelves those; everything else
in the layout is deliberately tool-free.

## AGENTS.md / CLAUDE.md routing

`AGENTS.md` (and `CLAUDE.md` where a repository carries one) routes agents
into the design system through the same door a designer uses: one line
pointing at `DESIGN.md` for UX guidelines. The agent entering through its
steer and the designer entering through the README arrive at the same index
and inherit the same chain.

## Inheritance

`design/Inheritance.md` names the layer, its parents in precedence order,
and what is explicitly overridden — see [notebook-layers.md](notebook-layers.md)
for the registration and inheritance contract. Undefined decisions inherit;
explicit local decisions override; cycles and unresolved overlaps are
invalid. The base chain every project inherits through is libyggterm's
`design/Inheritance.md` — the platform owns the base notebooks (owner,
2026-09-13: the reader ships nothing).

## emd and the extensible renderer

An `.emd` file is markdown plus typed fence extensions. The renderer,
`emd-renderer` in libyggterm, is **extensible by design**:

- **Base extensions ship in libyggterm.** Anything the whole fleet renders —
  plots, sparklines, metrics, datagrids, the interactive notebook embeds —
  is a base extension with its typed contract in the platform.
- **Private extensions stay private.** A product that needs a renderer
  extension it cannot publish grows it in its private modules checkout (the
  `yggui-priv-modules` pattern: one modular repo of renderer extensions
  composed by that product's pipeline) and names the dependency in the
  notebook that uses it.
- **Degradation is bounded.** A `.emd` fence rendered without its extension
  produces the bounded error card, never a crash and never silently
  dropped content. A notebook that needs a private extension says so in its
  front matter.

Extensibility lives in the libyggterm repository itself — the mechanism,
the registry and the base extensions. ydesign uses the mechanism to embed
interactive mini yggui/Dioxus apps in a notebook page; it does not fork the
renderer to do so.

## The ydesign skill and the registry

The ydesign repository ships `.agents/skills/ydesign/` — the navigation
skill for this layout, analogous to the memory system's entry gates. The
skill:

- routes a UX task through `DESIGN.md` doors — which notebook owns the
  decision, which layer inherits it;
- reads the full inheritance chain before any override;
- **registers the project it touches**: writes the project into the ydesign
  registry under the organized configuration area
  (`~/.yggterm/config/ydesign/projects.json`) and keeps its installed copy
  fresh at `~/.yggterm/skills/ydesign/`, so the skill is loadable on every
  host from one organized root;
- ensures the project's `DESIGN.md` names the skill door, so the next agent
  finds the navigation aid where it found the design system.

A registry is machine-local state and stays out of git, per
[notebook-layers.md](notebook-layers.md).

## Versioning

This contract is **ydesign 1.0.0**. A repository satisfies version 1.0.0
when it has: the layout tree above, a pointer-page `DESIGN.md`, the
`AGENTS.md` routing line, an `Inheritance.md` resolving to the base chain,
and — where the project has interactive pages — `design/notebooks/*.emd`
that render in ydesign without bounded errors. Additions (new asset kinds,
new base extensions, new skill verbs) extend the contract without breaking
it; changes that move or rename the doors bump the version.

## Migration

Migrate a repository in one lane: `git mv` existing design documents into
`design/` (preserving every rationale — migration is not permission to
redesign), write the pointer page, add the routing line, register with
`ydesign init`, and land with the repo's own checks green. Notebooks that
embed components move to `design/notebooks/` and take the `.emd` extension
when they actually carry emd fences; plain decision notebooks stay `.md`.
