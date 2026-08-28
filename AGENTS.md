# AGENTS.md

ydesign is the yggui base design language shipped as live notebooks — a
libyggterm Tier A document-surface app (OSC 7717 + loopback control server;
yggterm paints; this app declares and serves schemas).

**Repository licence: GPL-3.0-or-later (code), CC-BY-SA-4.0 (notebooks and
docs).** Do not introduce a second licence claim anywhere: README, NOTICE,
`Cargo.toml`, and file headers must agree, and every file under `notebooks/`
and `docs/` carries its `SPDX-License-Identifier: CC-BY-SA-4.0` banner.

## The product rule

The notebooks ARE the product. Their authority comes from being correct and
from being *live*: the examples pages append real painted widgets, so an
agent can screenshot the page and compare any surface against it. When you
change what the language says, change the notebook in the same commit as the
thing that changed.

## Working rules

1. **Port consumer.** This app is shaped on ytop (the first Tier A app).
   Platform mechanics — manifest, OSC declare, control server, schema
   vocabulary, lifecycle — follow `libyggterm-surfaces/SKILL.md` in the
   yggterm repo. Do not invent a second transport or a new widget kind
   without a forcing consumer.
2. **No libyggterm linking.** The app speaks the protocol; it does not link
   the platform. Keep it that way (it is a licence boundary, recorded in the
   IP register, and it keeps the dependency tree tiny).
3. **Base notebooks are source-controlled** under `notebooks/*.md`, embedded
   with `include_str!`. Stored/composed notebooks may never shadow a shipped
   id. Every notebook page opens with a heading; the shelf is checkable via
   `ydesign --notebook` without a GUI — keep it that way and extend the CLI
   reading when you add structure.
4. **emd fences must parse.** Component JSON in notebooks must satisfy the
   typed contracts in libyggterm's `emd-renderer/src/components.rs`
   (version 1, evidence blocks, bounds). An invalid fence renders a bounded
   error card — that is a shipped bug, not a soft failure.
5. **Privacy.** Invent every example: paths (`/home/user/proj`), titles
   (`3. widgets: refactor`), hosts (`example.test`). No real machines, no
   personal project names, no session transcripts. War stories cite
   symptoms and measured defects, never cases.
6. **Scratch space** is `~/.yggterm/scratchpad/` on fleet hosts — never
   `/tmp` (tmpfs/RAM).
7. **Tests travel with rules.** A notebook rule that the code enforces
   (unique ids, one page per notebook, banner stripping, shelf shape) has a
   unit test. `cargo test` green is part of every commit.

## The design consultation contract

Any design work in ANY fleet Web/GUI project: consult your project's own
design layer first, then this base set, then fall through the layers; grow
the lower layer when a component is missing; argue from pixel screenshots
(`server app screenshot`, or libyggterm's `gallery-shot.sh` for component
work). The Roadmap notebook is the ledger for filed defects and demanded
components — file there before fixing app-side.
