<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# DESIGN.md — ydesign, the base design language

This is the pointer page for this repository's design system. Doors, not
rooms: one line per door; the body lives in the file. The layout this file
follows is specified in [docs/design-layout.md](docs/design-layout.md)
(version 1.0.0) — every repository that ships UI carries this shape.

## The base notebooks (the design language, exhibited)

Read in the order below; [00-start-here](design/notebooks/00-start-here.emd)
defines the reading order and the evidence labels.

- [Start here](design/notebooks/00-start-here.emd) — the layer ladder, the reading order, the pixel-proof habit.
- [Foundations](design/notebooks/11-foundations.emd) — tokens, typography, colour, spacing: the numbers everything else inherits.
- [Component gallery](design/notebooks/01-component-gallery.emd) — the component vocabulary with live specimens.
- [Design catalogue](design/notebooks/08-design-catalogue.emd) — real fleet pixels, annotated with the choices that shaped them.
- [Sidebars](design/notebooks/02-sidebars.emd) — row engine, status vocabulary, partitioning, header anatomy.
- [Forms and settings](design/notebooks/03-forms-and-settings.emd) — the short-phrase rule and control-row anatomy.
- [Motion and feedback](design/notebooks/04-motion-and-feedback.emd) — blink clock, stage curtain, toast anchors.
- [emd and notebooks](design/notebooks/05-emd-and-notebooks.emd) — typed contracts for embedded evidence.
- [Ribbons](design/notebooks/09-ribbons.emd) — the Office-class ribbon pattern (the Excel target).
- [Complex sidebars](design/notebooks/10-complex-sidebars.emd) — the vault pattern: identity marks, credential chips, outcomes.
- [List views](design/notebooks/15-list-views.emd) — the interactive list-table, the default list container.
- [Icons](design/notebooks/14-icons.emd) — the icon system and its evidence.
- [ZCode reversed components](design/notebooks/12-zcode-reversed-components.emd) — the reversed-study reference pattern.
- [Worked examples](design/notebooks/06-worked-examples.emd) — real schemas as specimens.
- [Roadmap](design/notebooks/07-roadmap.emd) — demanded components and filed defects.

## The chain and the rules

- [Inheritance](design/Inheritance.md) — the base chain: Dioxus → yggui → app layer. Read the full parent chain before overriding.
- [Living design books](docs/living-design-books.md) — the book model: one row group per book, chapters as real Dioxus mini-apps.
- [Notebook layers](docs/notebook-layers.md) — project registration, assets, inheritance records.
- [The design/ layout](docs/design-layout.md) — the 1.0.0 contract this repo ships.

## Assets

- [assets/icons](design/assets/icons) — the base icon blocks (SVG).
- [assets](design/assets) — mock stills and catalogue captures cited by the notebooks.

## Navigation skill

- [ydesign skill](.agents/skills/ydesign/SKILL.md) — routes a UX task through these doors, registers projects, keeps the installed copy at `~/.yggterm/skills/ydesign/`. Installed copy: `~/.yggterm/skills/ydesign/SKILL.md`.

## The product itself

- The app (`ydesign`) renders the notebooks above as a document surface with a Guide | Examples shelf; the specimen mini-apps build in [specimens](specimens/).
- `AGENTS.md` routes agents here for UX guidelines; nobody needs the app to work in this repository — only `design/notebooks/*.emd` need ydesign to render.
