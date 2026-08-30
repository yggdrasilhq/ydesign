# @ygghq/ydesign

The yggui base design language — the layer every yggdrasil Web/GUI project
builds on — packaged as **data**: design tokens your project can read, and
the base design notebooks that exhibit the components and canonical patterns.

## Layers

```
L0 Dioxus primitives
L1 yggui + yggui-contract + emd-renderer   ← the language these tokens digest
L2 app design languages
L3 project overlays (your project's own notebook over the base)
```

## Consume

```js
import tokens from "@ygghq/ydesign/design-tokens.json";
// status.durable / density.rail / motion.blinkPeriodMs / emd.bounds …
```

Read `notebooks/` for the full language: the consultation ladder, the row
engine, sidebar partitioning, the short-phrase rule, motion laws, the emd
component contracts, and the roadmap ledger of filed defects and demanded
components.

## Ladder

1. Your project's own design layer (if it defines one).
2. These notebooks and tokens.
3. The layer below (yggui contracts, Dioxus).

Undefined at your layer → fall through. The layer below lacks a component →
**grow the lower layer** (file it in the Roadmap notebook); never hand-roll a
second encoding locally. Judge every visual decision from a pixel
screenshot.

## Licence

Design tokens: GPL-3.0-or-later. Notebooks (docs): CC-BY-SA-4.0.
The exhibits referenced by `%ASSETS%` paths render inside the `ydesign`
app; the text is complete without them.
