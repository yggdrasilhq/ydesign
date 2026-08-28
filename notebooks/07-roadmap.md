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
