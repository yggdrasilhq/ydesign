<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Start here — the base design language

ydesign is the *base* design language of the yggdrasil app fleet, shipped as
live notebooks. "Base" because every app builds over it. The layering is the
whole point:

| Layer | What it is | Where it lives |
|---|---|---|
| **L0 · Dioxus** | UI primitives | upstream |
| **L1 · yggui** | The component vocabulary, the type system, the contracts | libyggterm (`crates/yggui`, `yggui-contract`, `emd-renderer`) |
| **L2 · App languages** | yedit, ychrome, ytop, kasten — each app's own patterns over L1 | each app's repo |
| **L3 · Project overlays** | A product's own design notebook, layered over L1 | that product's repo |

## The consultation ladder

Work at your own layer first. When your layer does not answer a question:

1. **Fall through one layer** — read the same concern in the layer below.
   An app that cannot decide a row's spacing consults the row engine (L1),
   never invents a number (L2).
2. **If the lower layer lacks the component, grow the lower layer.** A new
   component belongs in `yggui` (with a gallery entry in the *Component
   gallery* notebook), not hand-rolled inside the app that wanted it. The
   admission gate is the platform's own: a forcing consumer, and a second
   consumer before the widget becomes a vocabulary kind.
3. **Never ship a second encoding of a decided thing.** If a rule exists —
   one segmented control, one field skin, one row engine, one status
   vocabulary — a second implementation is the bug, however local it feels.

## The working habit: argue from pixels

A design judgement is argued from a **pixel screenshot**, never from source
and never from a description. The instruments:

- Component work: render the gallery example and shoot it —
  `cargo run -p yggui --example conversation_gallery`, then
  `libyggterm/scripts/gallery-shot.sh <example> out.png` (headless sway +
  grim; build the example *first*, outside the capture window).
- Live surfaces: `server app screenshot <out.png>` against the running
  shell. For document surfaces and splits use the faithful backend
  (`--backend os`); the composite paste hides DOM layers.
- Judge, fix at the **component layer**, and re-shoot. "Ugly" that is fixed
  in the call site comes back in the next call site.

## Where authority lives

- **yggterm `DESIGN.md`** — the prose constitution: brand intent, control
  language, status vocabulary, motion, the settled failure stories.
- **These notebooks** — the same language, *exhibited*: real components,
  real schemas, patterns you can screenshot and compare against.
- **libyggterm specs** — `docs/spec-app-architecture.md` (the tier decision:
  who paints the pixels) and `docs/spec-emd-renderer.md` (what a document is).
- **`.agents/skills/libyggterm-surfaces/SKILL.md`** — the app-platform
  contract: surfaces, transport, lifecycle, the widget vocabulary.

When a notebook and `DESIGN.md` disagree, fix the drifted one — and record
the correction in the notebook, because notebooks are what the next agent
reads first.

## How other apps consume this

An app's campaign memory points at these notebooks (the shelf ships inside
the app: run `ydesign` inside yggterm). A project that defines its own layer
(L3) writes its own design notebook in its own repo, states what it overrides,
and defers to this base set for everything it does not. The fallback is
always the adjacent lower layer — never a local invention.

## What each notebook covers

- **Component gallery** — every component, live, with the one-owner rule it
  encodes.
- **Sidebars** — the row engine, densities, status dots, partitioning.
- **Forms & settings** — section cards, the one field skin, the short-phrase
  rule.
- **Motion & feedback** — toasts, the stage curtain, the shared blink clock.
- **emd & notebooks** — the extended-markdown contracts and authoring rules.
- **Worked examples** — canonical surfaces rebuilt as live schemas.
- **Roadmap** — demanded components, each with its forcing consumer.
