<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Iconography — why ZCode's icons are crisp, and the yggui icon-set proposal

The owner's verdict: "yggui needs to have an iconfont. The icons of ZCode look
crisp. But the yggterm iconography leaves much to be desired." This page
measures what each surface actually renders today, isolates why ZCode reads
crisp, and proposes the one-owner fix. Measured 2026-09-06.

## What each surface renders today (observed)

| Surface | Icon system today | Evidence |
|---|---|---|
| libyggterm `yggui` core | **unicode text glyphs** — `↑ ↓ → ← ×` | grep of crates/yggui: zero `<svg`, zero `viewBox` |
| `chrome.rs` window controls | text glyphs (minimise/maximise/close strings) | chrome.rs |
| yggui-chat (jyas webapp) | **13 lucide constants as complete inline svgs**, stroke 1.5, currentColor, wrapper-sized | the yggui-chat crate, `src/icons.rs` |
| practice-rs webapp | none found — glyph/text only | zero `viewBox` in app sources |
| ymacs | CL side, glyph-based by nature | out of scope for the wasm path |
| **ZCode** | **bespoke inlined set: lucide fragments recombined + bespoke glyphs**, rendered 16px, stroke **1.5**, round caps, currentColor, fill none; plus **lucide-react v1.17.0** bundled for plugin `DynamicIcon` (~27 icons fingerprinted by path data: bath, bot, coffee, chess-rook, cooking-pot, globe, wand-sparkles…) | zcodereversed mining 2026-09-06 |
| t3code | lucide-react ^0.564.0 stock (stroke 2.0 default) | package.json |

## Why ZCode's icons look crisp (observed)

1. **Stroke 1.5, not 2.0.** lucide's 24-grid paths are drawn for a 2px stroke
   at 24px; at the 14–16px boxes real UIs use, 2.0 strokes visually blob.
   ZCode re-renders at `stroke-width="1.5"` and gains a full stop of clarity.
   (So does yggui-chat — it already copies this recipe.)
2. **One optical box.** Everything lands on a 16px wrapper with `currentColor`,
   `fill:none`, round caps/joins — icons inherit text colour and weight.
3. **Fragments recombined, not re-drawn.** ZCode's custom glyphs (agent mode
   icons etc.) are lucide path fragments recombined with bespoke paths — same
   grid, same terminals, so the set reads as one family.

## The gap (observed)

- **No shared icon source.** One crate holds 13 good icons; everything else is
  ad-hoc glyphs. A new app re-inlines by hand or reaches for unicode arrows.
- **Unicode arrows are the visible defect** — `↑↓→←×` render with the ambient
  text font, so weight, width and vertical centering change per platform/font.
  That is most of the "leaves much to be desired".
- **No naming convention**, so the same glyph gets re-picked per app.

## Proposal (proposed — needs owner GO before it grows a crate)

An **icon set, not an icon font.** Fonts can't do multi-tone, scale poorly in
wasm, hurt screen readers, and force a glyph→codepoint memorisation layer. The
want behind "iconfont" — *one place, one name, crisp everywhere* — is satisfied
by one crate of inline-svg constants plus one sizing convention:

1. **`yggui-icons`** (or promote `yggui-chat/src/icons.rs`): lucide ISC paths
   as `pub const` full svgs, stroke-width **1.5**, round caps, `currentColor`,
   named lucide-kebab → `SCREAMING_SNAKE` (`ARROW_UP`, `SQUARE_PEN`, …).
2. **Sizing contract**: wrapper classes `i12/i14/i16/i20`; the svg is
   width/height 100% inside the wrapper (the mini-lab `.i16` pattern).
3. **Arrows rule**: replace unicode `↑↓→←×` in `yggui` core (chat_input send,
   dpad, split_button caret, chrome window controls) with set equivalents at
   text size — one PR, visible fleet-wide improvement.
4. **One-owner enforcement**: after the set lands, a new inline `<svg` in an
   app layer is the defect; grep-able.
5. **Recombine, don't re-draw**: bespoke glyphs (agent/tool mode marks) copy
   ZCode's trick — lucide fragments on the same grid, same terminals.

Forcing consumers: jyas-webapp (already consumes 13), mini-lab icon row
(live), and ychrome's toolbar as the second consumer that admits it into the
schema vocabulary.
