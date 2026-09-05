<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Roadmap — demanded components

Components the apps have demanded, each with its forcing consumer and the
layer that grows. This page is the design system's *defect and demand
ledger*: "the sidebar needs polish" becomes a filed, scoped, owned row.

The admission rule is the platform's own: a component enters the vocabulary
with a **forcing consumer**, and becomes a schema kind when a **second**
consumer wants it. Between those two points it lives in the app that needed
it first — visibly, here, so the next app finds it instead of reinventing it.

## The queue

| Demand | Who needs it | Layer that grows | State |
|---|---|---|---|
| **Left-ruler scroll component** — marks on a vertical rail (user turns in a transcript; months/years in a photo library; headings as bookmarks) | transcripts, media, readers | `yggui` — one component owning rail geometry; hosts supply `{offset, depth, label}` | Spec'd (see *emd & notebooks*); first host lands it |
| **Plot-quality charts** — ggplot2/plotly-grade statistical plots as components | telemetry notebooks, analytics | emd `plot` grows facets/scales; the renderer owns layout | `plot` exists (line/area/bar/point/step); quality bar tracked here |
| **Full-width devtools mini-app** — a network-timing inspector as one notebook block, scrollable, reusable | web development inside notebooks | emd component composed of `panel` + `datagrid` + `plot`, driven live by a document-version refresh | Demanded |
| **Half-width "modern top"** — a top-style live block that does NOT span the notebook | telemetry | emd `grid` composition of `sparkline` + `metric` | Demanded |
| **Full-width calendar** — Notion-class month component | knowledge app | emd component (new `calendar` kind, Tier C: admitted when its second consumer lands) | Demanded |
| **Axiom-class log/query views** — devtools-grade filtering over logs | log viewers, tracing | emd `query` + `datagrid` grow saved views | Demanded |

## Notebook review additions

| Finding | Owner and acceptance | State |
|---|---|---|
| Detached ribbon panel, dark tab slab and weak command groups | Shared ribbon renderer; continuous shell tabs, explicit pinned/temporary modes, grouped commands, keyboard and pixel proof | Proposed anatomy in Ribbons; component change pending |
| Vault setup repeated across pages; generic credential bubbles obscure site identity | Browser sidebar content plus shared row renderer; contextual setup, site marks, distinct credential labels, short Fill/details paths | Measured row: 264 × 38, icon 15, bubble 26; proposal in Complex sidebars, not shipped |
| EMD panel controls rendered disabled | Document host's panel-action routing; demonstrate a real state-changing control before calling it interactive | Filed; current notebooks use supported schema-row exercises |
| Notebook images capped at 560 × 320 | Document image renderer; distinguish a reading illustration from an inspectable full-width design specimen | Measured in shadow; diagrams redrawn to this cap so labels stay readable. Full-width/zoom affordance remains pending |

The exercises model transitions, not production ribbon or vault components.
Passing their actions does not validate a repaired component. Private runtime
captures stay outside this public repository; published studies use invented data.

## The first filed defects

The design system opens its ledger with the two defects that motivated it.
Both are recorded, not fixed, so the fix lands with pixel proof at the
component layer:

1. **Settings-rail top rows** — explanatory prose sat *beside* the toggles as
   multi-line paragraphs: control row proportions broken, toggle stranded in
   dead space. Fix = the **short-phrase rule** (one muted phrase, under the
   control, never a paragraph beside it). See *Forms & settings*.
2. **A browser tab rail's header** — the profile pill and `+` ride the
   heading as solid accent fills, stacked under the nav row and omnibox:
   three bands of loud before any row. Fix = the **header anatomy** standard
   (title row → tool row → section heading) with header actions at ICON
   weight — muted, accent on hover only (`session_row_action_button_style`),
   matching the yedit files header; the accent budget belongs to the rows'
   own states. Code: yggterm-shell `right_rail.rs` `WebTabsRailBody`
   (RailHeader actions, ~line 1554). ⚠ Owned by yggterm (WebTabs rail is
   yggterm chrome), but coordinate with any active ychrome-ux lane before
   touching it. See *Sidebars*.

## How this page grows

An agent that meets an ugly surface files the divergence here (or in the
app's own overlay notebook) *before* fixing it: what is wrong, which rule it
breaks, the screenshot pair. A defect with a name and a rule stops being a
thousand small tweaks — it becomes one row, one owner, one fix.
