<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Iconography audit — the @ygghq app layers, 2026-09-14

The one-owner law (`crates/yggui-icons` in libyggterm): **"A new inline `<svg`
in an app layer is the defect this crate exists to prevent."** This audit
swept every UI repo in the ydesign registry for inline svg iconography and
crate adoption. Method: grep `svg` / `dangerous_inner_html` across each
repo's app crates, classify every hit (iconography defect vs brand/content/
base-layer legitimate), and compare duplicated path data against the crate
constants byte-for-byte.

## Findings and disposition

| Repo | Site | Class | Disposition |
|---|---|---|---|
| yggui-priv-modules (`yggui-chat`) | `src/icons.rs` (13 lucide constants, 9 never used) wired via `include!` in sidebar.rs | duplicate encoding, in a SHARED-layer crate | **LANDED** `a2798d8`: crate path dep, `use yggui_icons as icons;`, the copy deleted |
| jyas-webapp | `main.rs` six `dangerous_inner_html` lucide literals (PENCIL, SHARE, SETTINGS, COPY, ROTATE_CW, GIT_BRANCH) | duplicate encoding, app layer | **LANDED** `cc1761a`: crate constants; path data verified identical pre-swap, so rendering is unchanged |
| jyas-webapp | `MdBlock::Table` pattern missing emd-renderer's new `alignments` field | stale pattern match — jyas main did not compile against current libyggterm (pre-existing break) | unblocked in `cc1761a` (`alignments: _`, rendering unchanged); honoring alignments in the prose table renderer is the jyas follow-up |
| jyas-webapp | `GoogleIcon` / `AppleIcon` (multi-colour fill) | brand marks are content, not stroke iconography | legitimate, stays local |
| yggterm | `yggterm-shell` inline `svg {}` iconography: `sidebar.rs` RowDisclosureChevron (bespoke 12-grid, stroke 1.35 — deliberately ONE shared chevron) + ~5 more sidebar sites, `overlays.rs` ×2, `viewport.rs`, `right_rail.rs` | app-layer inline iconography, pre-crate | **FILED** in yggterm `docs/pending-bugs.md` (adoption lane; the crate was born from yggterm's arrows — the shell never finished adopting it) |
| yggterm | `yggterm-core/src/icon.rs` — brand app-icon install assets (svg/png pixmaps) | brand assets, not UI iconography | legitimate |
| practice-rs | `practice-core` chart/diagram/score_report + `practice-api` time-pressure/review-pie svg generators | data-visualization content | legitimate (generated charts, not iconography) |
| ytop, zcode-tui | no svg iconography found | — | clean |
| libyggterm | the crate + `Icon` component; books and specimens embed svg as teaching content | base-layer owner | clean |

## State of adoption

- Before today **no app layer depended on the crate**. Now yggui-chat (and
  every consumer of yggui-priv) and jyas-webapp do. Deleted copies: 13
  (yggui-chat, 9 of them dead) + 6 (jyas) = 19 duplicated icon definitions;
  the SSOT is one file, `crates/yggui-icons/src/lib.rs`.
- `yggui` consumes `yggui-icons` internally (`dpad.rs`) but does not
  re-export it, so consumers take the path dep directly (the pattern yggui
  itself uses). **Recommended base-layer change:** a `pub use yggui_icons;`
  re-export in `yggui` would make app-layer adoption a one-liner. Left to
  the base layer's own flow rather than widened mid-audit.
- The bespoke RowDisclosureChevron in yggterm-shell is the model citizen of
  the OLD world: one local glyph, one shared definition, a doc comment
  saying so. The adoption lane should graduate it (and the remaining shell
  glyphs) into the crate with its measured rationale intact — not flatten
  it into a 24-grid lookalike.

## Proof

- yggui-chat: `cargo check` + `cargo test` green in the lane; the priv
  modules repo has no ygg-ci recipe (small debt, worth tuning).
- jyas: `cargo check` green after the unblock; live deploy to jyas.gour.top
  follows the landing per the deploy law (see the session door for the
  deploy record).
- The jyas icon swap is render-identical: every swapped literal's path data
  was compared byte-for-byte against the crate constant before the swap
  (attribute quoting differs — `"` vs `'` — which SVG parsing is agnostic
  to; path data is what renders).

Audit ride-along: the `ydesign forget` verb (dream ACK-21e65d98ee) shipped
in the same lane — the state-reset verb the dream asked for, proved on both
paths (file present → `forgot` + shelf; file absent → `already fresh` +
shelf).
