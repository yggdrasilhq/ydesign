# ydesign — the yggui base design language, as live notebooks

A libyggterm document-surface app for [yggterm](https://github.com/yggdrasilhq/yggterm):
it opens yggterm's viewport as a set of **design notebooks** that exhibit the
fleet's base design language — the `yggui` component system, the canonical
sidebar patterns, form rules, motion laws, and the extended-markdown
(`emd`) contracts — as real, screenshot-able surfaces.

Every Web/GUI project builds on one design language, in layers:

```text
L0  Dioxus primitives
L1  yggui + yggui-contract + emd-renderer   ← this app documents and exhibits L1
L2  app design languages (yedit, ychrome, ytop, kasten, …)
L3  project overlays (a product's own design notebook over L1)
```

Consult your own layer; fall through to the layer below when undefined; when
the layer below lacks a component, **grow the lower layer** — never a local
hand-roll. Judge every visual decision from a **pixel screenshot**.

## The notebooks

| Notebook | What it covers |
|---|---|
| Start here | The layer ladder, the consultation ladder, the pixel-proof habit |
| Component gallery | Every yggui component and schema widget, with live specimens |
| Sidebars | The row engine, status vocabulary, partitioning, header anatomy |
| Forms & settings | Section cards, the one field skin, the short-phrase rule |
| Motion & feedback | Toasts and anchors, the stage curtain, the shared blink clock |
| emd & notebooks | The extended-markdown contracts and authoring rules |
| Worked examples | Canonical surfaces rebuilt as live schemas to compare against |
| Roadmap | Demanded components and filed defects, each with its forcing consumer |

## Running

Inside yggterm (thin-client surface mode — the daemon exports
`YGGTERM_SESSION_ID`):

```sh
ydesign                 # opens the notebooks as a document surface
```

Outside yggterm the same command prints the shelf instead of opening a
half-surface. The notebooks are checkable without a GUI:

```sh
ydesign --notebook                 # list the shelf
ydesign --notebook sidebars --page 1
ydesign --once --json
```

## Architecture (the short version)

ydesign is a Tier A schema app: it declares its surfaces over **OSC 7717**
on the terminal byte stream and serves widget **schemas** from a loopback
control endpoint; yggterm paints them. It links no libyggterm code — the
MPL-2.0 platform stays an arm's length away by construction. Base notebooks
are compiled into the binary from `notebooks/*.md`; agent-composed notebooks
would live under `~/.local/share/ydesign/notebooks` and may never shadow a
shipped id.

Read `.agents` knowledge in the yggterm repo
(`.agents/skills/libyggterm-surfaces/SKILL.md`) before extending the
surfaces, and libyggterm's `docs/spec-app-architecture.md` before choosing a
content tier.

## Licence

Code: **GPL-3.0-or-later** (`LICENSE`).
Notebooks and docs (`notebooks/`, `docs/`): **CC-BY-SA-4.0**
(`LICENSE-CC-BY-SA-4.0`).
Copyright 2026 Avikalpa Kundu. See `NOTICE` and `THIRD-PARTY-NOTICES.md`.
