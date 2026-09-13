<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# DESIGN.md — ydesign, the reader

This repository is the READER for the fleet's design system: the `ydesign`
app, the navigation skill, and the registry mechanics. By the no-shipping
law (owner, 2026-09-13) it ships NO base notebooks — the design language
lives in the repositories that own the components and surfaces, and the
registry assembles the shelf. The layout contract this system follows is
[docs/design-layout.md](docs/design-layout.md).

## Where the design language lives (the doors)

- [libyggterm design notebooks](https://github.com/yggdrasilhq/libyggterm/tree/main/design/notebooks) — the base: foundations, component gallery, sidebars, forms, motion, emd, ribbons, complex sidebars, icons, list views, roadmap. The component vocabulary is libyggterm's.
- [yggterm design notebooks](https://github.com/yggdrasilhq/yggterm/tree/main/design/notebooks) — the chrome books: worked examples and the design catalogue.
- Product and study repos carry their own `design/notebooks/*.emd` (practice-rs, jyas, zcodereversed, t3codereversed, yggui-priv-modules).
- [libyggterm Inheritance](https://github.com/yggdrasilhq/libyggterm/blob/main/design/Inheritance.md) — the base chain: Dioxus → yggui → app language → project overlay.

## This repo's doors

- [docs/design-layout](docs/design-layout.md) — the layout contract (version 1.1.0): every UI repo carries DESIGN.md + design/.
- [docs/spec-ydesign](docs/spec-ydesign.md) — the app spec.
- [docs/notebook-layers](docs/notebook-layers.md) — registration, assets, inheritance records.
- [docs/living-design-books](docs/living-design-books.md) — the book model.
- [Navigation skill](.agents/skills/ydesign/SKILL.md) — routes UX tasks, registers projects, keeps the installed copy at `~/.yggterm/skills/ydesign/`.
- [design/Inheritance](design/Inheritance.md) — this repo's own chain record.

## The registry

`~/.yggterm/config/ydesign/projects.json` (per host, machine-local, never in
git) names the repositories whose `design/` trees appear on the shelf. A
fresh host onboards by registering the repos — the skill does this.
