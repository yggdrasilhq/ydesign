# Spec — ydesign, the design-language app

Recorded 2026-08-28, owner-directed. ydesign is the yggui base design
language made visible: a libyggterm document-surface app whose notebooks are
the base design system the fleet's Web/GUI projects consult.

## What it is

- A Tier A schema app shaped on ytop: OSC 7717 `sidebar ; declare`, loopback
  control server (`GET /ping`, `GET /pane/{design,rail}`, `POST /action`),
  heartbeat ~4s, close on SIGINT. No libyggterm linking.
- One viewport pane (`design`) rendering the open notebook page; one rail
  pane (`rail`) carrying the notebook shelf.
- An app-declared titlebar switch: **Guide | Examples** — the guideline
  shelf and the worked-examples shelf are the two reading modes.
- Eight base notebooks compiled into the binary from `notebooks/*.md`
  (CC-BY-SA-4.0); the exhibition pages (`gallery`, `examples`) append real
  composed widgets below their prose so the page is a screenshot-able
  specimen, not a description.

## What it does NOT cover

- It does not render anything itself. It authors schemas; yggterm paints.
- It does not replace DESIGN.md. The notebooks exhibit and point at the
  prose constitutions; where they disagree, fix the drifted layer and record
  the correction.
- It does not decide app-specific design. App layers (L2) and project
  overlays (L3) own their patterns; this base set owns the vocabulary and
  the canonical patterns beneath them.
- It ships no component code. Demanded components are grown in libyggterm
  (or the host, for schema kinds), per the two-consumer admission gate, and
  tracked in the Roadmap notebook.

## Content rules

- Every notebook page opens with a `# heading`; one page per notebook at
  v1 (the model keeps `pages` for growth).
- emd fences must satisfy the typed contracts (version 1, evidence blocks,
  bounds) — an invalid fence is a shipped bug.
- Privacy: invented examples only; war stories cite symptoms and measured
  defects, never live cases.

## Licence split (Launch Gate Step 0, fingraph analysis)

- Class **F** (fame/open), public in `yggdrasilhq`.
- Talks to libyggterm over the control protocol; does not link it — MPL-2.0
  does not reach this repo. GPL-3.0-or-later for code from commit zero.
- The notebooks/docs are the product's documentation layer: CC-BY-SA-4.0,
  stated in every notebook's SPDX banner, README, NOTICE, and the manifest.
- Publishes nothing owner-only: fixtures in the notebooks are invented, and
  analytical examples cite generic sources.

## Acceptance

- `cargo test`: shelf shape, unique ids, banner stripping, licence-split
  constants, wire envelope compatibility.
- `ydesign --notebook` lists the shelf without a GUI; each page prints.
- Inside yggterm: both panes declare, the switch flips modes, page turns
  work, specimen controls answer with an echo notice, SIGINT retires the
  surface.
- A pixel screenshot of the exhibition pages exists as the first acceptance
  artefact of the visual pass.
