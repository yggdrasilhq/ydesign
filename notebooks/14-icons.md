<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Icons — the one icon source

Every surface reaches for the same glyphs through **one crate**; this page is
its contract and its specimen index. Label: **measured** for the recipe,
**enforced** for the one-owner rule.

## The crispness recipe (measured)

lucide's 24-grid paths, rendered at the wrapper size, with three departures
from stock usage that make icons read crisp at UI sizes:

1. **`stroke-width="1.5"`** — lucide's default 2.0 blobs below 20px; 1.5 is
   the difference between crisp and fuzzy (the desktop references render
   exactly this).
2. **One optical box** — a flex-centered wrapper (`i12/i14/i16/i20`), svg at
   100% of it, `currentColor`, `fill:none`, round caps and joins.
3. **Recombine, don't re-draw** — bespoke glyphs are lucide fragments on the
   same grid with the same terminals, so the set reads as one family.

## The crate (yggui · L1)

| Piece | The rule it owns |
|---|---|
| `yggui-icons` consts (`ARROW_UP`, `COPY`, …) | The only icon source. A new inline `<svg` in an app layer is the defect this crate exists to prevent — grep-able |
| `Icon { icon, size }` | The wrapper geometry: flex-centered, exact px, `flex:none`, `color:inherit` |
| `glyph()`-style text fallbacks | Kept for a11y labels and tests; the RENDER is always the icon |

Provenance: lucide (ISC) paths; crate code carries the workspace licence.

## Unicode arrows are the defect class (enforced)

`↑ ↓ → ← ×` render with the ambient text font — weight, width and vertical
centering change per platform. Core surfaces that carried them now render
set equivalents (dpad, find steppers, close buttons). Review rule: a text
arrow in a control is a finding, not a style.

## Specimens

The icon row and every icon-bearing control in the component testbed app
render from the crate at 1.5 stroke; the composer, sidebar rows and rail
sections show them in context at 14–16px.
