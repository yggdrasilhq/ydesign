<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# ZCode reversed, components & colour (measured reference)

**edition 2026-09-07 · rev 1**

Two measured design references now exist as repos, and one living dioxus
testbed proves they port. This notebook is the *reading guide* to them; the
repos are the evidence.

| Reference | Repo | What it is | Licence posture |
|---|---|---|---|
| ZCode desktop 3.11.2 | zcodereversed (private fleet forgejo) | stylesheets + tokens + component anatomy extracted from the asar | first-party asset extraction (our own product) |
| t3code | t3codereversed (private fleet forgejo; mirrors upstream e60821f0e) | verbatim `index.css` + 49-component ui kit | MIT, no RE, attribution in NOTICE |
| mini-lab | `t3codereversed/miniapp/`, served on a fleet host | dioxus 0.7 wasm gallery: one component vocabulary, four token arms | ours |

Label: everything in the first two sections is **observed**; the Dioxus-path
section is **proposed** until the port lands app-by-app.

## What ZCode actually is (observed)

React 19 + **shadcn/ui** (new-york, v4-era, `data-slot` markers) + Tailwind
v4 oklab + Radix behaviours + cva variant maps. **104 distinct `data-slot`
components** inventoried from the renderer chunks:

alert · alert-dialog · avatar · badge · button · card · chart · checkbox ·
collapsible · command · context-menu · dialog · dropdown-menu · hover-card ·
input-group · kbd · label · progress · scroll-area · select · switch · tabs ·
textarea · tooltip (plus their `-*` parts).

### The measured anatomy (the "outstanding amalgation", in numbers)

| Component | Measured recipe |
|---|---|
| select/trigger sizes | xs **h-5** · sm **h-6** · default **h-7** · lg **h-8**, radius grows with size |
| variant map (shared control language) | `input` · `default` (bg-primary) · `outline` · `secondary` · `ghost` (hover:bg-hover) · `destructive` |
| card | rounded-xl, border-card-border, gap-4 py-4, text-ui-base/relaxed |
| dropdown-menu content | rounded-xl, bg-menu, p-1, gap-0.5, radix motion 100ms |
| tooltip content | rounded-lg, bg-tooltip, px-3 py-1.5, text-ui-sm, kbd-aware padding |
| dialog content | rounded-xl, bg-popover, p-4, fade+zoom-95, `[app-region:no-drag]`, `platform-linux-desktop:` top offset |
| command | rounded-xl bg-popover p-1; item min-h-7 rounded-lg px-2.5, hover/selected bg-menu-hover |
| switch | 18×32 (sm 16×28), thumb 14px, track = primary at 30% → checked primary |
| checkbox | 16px rounded-sm, checked = bg-primary |
| badge | default/secondary/destructive(**/10 soft**)/outline/ghost/link |
| tabs-trigger | data-active pattern, vertical + line variants, h-fills-list |

### The ZCode flavour on top of stock shadcn (observed)

1. **Compact control scale**, h-5…h-8 (20–32px), not shadcn's h-9/h-10. Denser
   than ChatGPT; this is the "pro but dense" feel.
2. **A text scale tied to the user**: `text-ui-base`, `text-ui-sm`,
   `--ui-font-size: 14px`, every control scales with one root variable.
3. **Semantic tokens only**, components never name a palette colour; they say
   `bg-hover`, `bg-menu`, `border-input-border`. The theme arms decide values.
4. **Desktop-shell awareness**, `app-region:no-drag` and a
   `platform-linux-desktop` variant inside component recipes.
5. **Triggers show state**, `aria-expanded:bg-selected` on selects/dropdowns.

## The colour system (observed, full tables in repo FINDINGS.md)

Four arms cascade in the main bundle, later wins:

`:root` (tailwind light) → `.dark` (tailwind dark) → `.theme-zai-light` →
`.theme-zai-dark`, the **zai arms are the shipped look** (the runtime toggles
`dark`+`theme-zai-dark` / `theme-zai-light` together).

| Token | zai-light | zai-dark |
|---|---|---|
| background | `#f8f8f8` | `#161616` |
| header/panel/tab | `#fff` / `#fff` / `#f0f0f0` | `#202020` / `#202020` / `#202020` |
| sidebar | `#f0f0f0` | `#161616` |
| card / popover / input | `#fff` | `#2b2b2b` |
| surface / hover | ink 3% / 5% | white 5% / hover 5% |
| border / hover | ink 10% / 15% | white 10% / 15% |
| **brand** | **`#000` ink** | **`#fff` ink** |
| accent | `#ebf4ff` | `#001d3d` |
| foreground | neutral-800 | neutral-300 |
| trajectory | `#2563eb #0f766e #7c3aed #d97706 #0284c7` | `#60a5fa #2dd4bf #a78bfa #f59e0b #38bdf8` |

The signature decision: **brand = ink**, and accents are pale washes
(`#ebf4ff`), so colour carries almost no chrome, the poshness is neutral
surfaces + one accent + crisp hairlines. (The jyas dark fold of 2026-09-05
keyed the generic `.dark` arm, neutral-900, not the shipped zai-dark;
re-folding is queued in the jyas DESIGN.md orbit.)

t3code's answer to the same brief, for contrast: light zinc-25 canvas + blue
oklch(0.588 0.217 264) primary; dark **pure #000** with white-alpha rows
(8/11/7%); glass (blur 12/16px, saturate 1.1) + feTurbulence grain at 3.5%
opacity; 52px topbar.

## The Dioxus path (proposed)

The kit is **class strings + token sheet**, no runtime dependency on React.
So the transform is mechanical, and mini-lab proves it:

1. Copy the token arms verbatim into one stylesheet (mini-lab
   `assets/tokens.css`: `.t3-light/.t3-dark/.zai-light/.zai-dark`).
2. Re-express each shadcn recipe as plain CSS classes over those tokens
   (`assets/gallery.css`), one component vocabulary, four arms.
3. Re-implement the Radix behaviours as small Dioxus hooks (open/close,
   focus trap, outside-dismiss, command filter), **this is the real port
   cost**; the visuals port for free.
4. Keep the one-owner rule: shared components land in the yggui layer;
   app-specific wiring stays in the app layer.

Port order by jyas-webapp value: button/input/textarea/badge/card/kbd →
tabs → dropdown + tooltip → dialog/sheet → command palette.

**Testbed**: `t3codereversed/miniapp`, `dx build --platform web`
(dioxus-cli 0.7.10 on the build host), serve the `public/` dir. Every section
changes real state; the theme picker swaps all four arms live.
