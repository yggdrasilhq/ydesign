<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Chapter inventory — the conversion ledger

[Living design books](living-design-books.md) requires "a chapter inventory
with explicit status," and forbids calling the collection converted while any
chapter remains an illustration. This file is that inventory. It is reviewed
like code: when a chapter's status changes, change the row in the same commit
that changed the chapter.

## Status vocabulary

| Status | Means |
|---|---|
| `front matter` | Purpose, tokens, procedure or governance. Prose is the correct form; the specimen rules do not apply. |
| `legacy illustration` | Explains a component with prose and/or a still or reconstructed image, optionally a schema state exercise. Fails the living-books acceptance bar. |
| `working specimen` | A real component demo with deterministic fixtures, real state transitions and a complete Reset is reachable from the chapter — in-host, or from a named external target while the host mounting path is unresolved. |
| `reviewed reference` | A working specimen whose task completion, keyboard focus, reset and narrow/zoom reading have a recorded human review, linked to the edition. |

Labels on evidence everywhere else mean: **observed** (measured from a real
surface), **reconstructed** (redrawn to teach; invented content), **proposed**
(a design offered for comment), **verified** (a claim re-checked against the
artifact on a stated date).

Edition stamps: every book carries `edition <date> · rev <n>` under its
heading. A critique names the edition it reviews; an accepted critique bumps
the rev and links the verification.

## The base shelf (ydesign, L1–L2)

| Book | Chapters | Status | Specimen / next proof |
|---|---|---|---|
| Start here | layer ladder, consultation, pixel habit, status labels, reading order | `front matter` | — |
| Foundations | semantic colour, typography, relationships | `front matter` (token SSOT: `npm/design-tokens.json`) | — |
| Component gallery | yggui + schema + emd index; live row appendix | `front matter` + `working specimen` (list-row rows) | Remaining vocabulary is indexed, not exhibited; per-component chapters graduate into their own books |
| Catalogue | forms rail, emd reader, live editor, notebook page | `front matter` + observed pixels | Screenshots are evidence, not specimens; they seed future chapters |
| Sidebars | row vocabulary · status vocabulary · partitioning · header anatomy · chrome behaviour · proving | `working specimen` (row engine, via the live row appendix and Worked examples) · rest `legacy illustration` | vault-study.svg is reconstructed; partition/header chapters want in-host specimens |
| Forms & settings | section cards · short-phrase rule · one field skin · stored value · control choices | short-phrase rule `reviewed reference` (the fixed yggterm Settings rail; accepted before/after pair 2026-08-28) · rest `legacy illustration` | Field skin and stored-value want a live field specimen |
| Motion & feedback | durations · toasts · stage curtain · blink clock · auto-hide chrome · proving | `legacy illustration` (with observed evidence recorded in yggterm's history) | Motion needs its burst-screenshot specimen per rule |
| emd & notebooks | the model · contracts · worked example · notebooks pattern · left-ruler demand | worked example `working specimen` (the fence renders live) · rest `front matter` | Left-ruler: demanded, see Roadmap |
| Worked examples (Examples mode) | Live Sessions anatomy · partitioned sidebar · the comparison loop | `working specimen` (schema rows, host-painted) | Schema-level, not component-level; conversion waits for the mounting path |
| Roadmap | demands, review additions, filed defects | `front matter` (ledger) | — |
| Ribbons | rejected vs proposed anatomy · keyboard · review jobs · exercise | `legacy illustration` (reconstructed diagram + schema state exercise) | Needs the shared ribbon renderer change; review tables ready for its owner |
| Complex sidebars | identity anatomy · one job per level · action counting · walkthrough · states · verification | `legacy illustration` (reconstructed + one measured state; schema exercise) | Shared row renderer + real vault flows; measured row 264×38 recorded |
| List views | contents-page failure · three lists · editorial anatomy · interaction · mini-app · gates | `working specimen` (staging: `specimens/` ChapterList, deterministic fixtures + reset + critique draft) · in-host page `legacy illustration` | Mount the specimen inline; the renderer repair in yggterm-shell is the owner's pending change |
| Icons | crispness recipe · the crate · arrows defect · specimens | `working specimen` (renders from `yggui-icons` in the host gallery) | Reviewed pending: a pixel pass at 12/14/16/20 px |
| Inheritance | the base chain, override scope | `front matter` (governance) | — |
| ZCode reversed | what ZCode is · measured anatomy · colour system · Dioxus path | `working specimen` (external: mini-lab.gour.top, registry `zcodereversed/design/specimens.json`) · colour/anatomy claims `observed` | External while the mounting path is unresolved; reviewed pending per chapter |
| (retired) Iconfont → Icons | — | merged into **Icons** 2026-09-07 | The iconfont proposal became the `yggui-icons` crate; the decision record lives in Icons §"Why not a font" |

## Registered project layers (L3, read through the registry)

| Project | Book(s) | Status | Next proof |
|---|---|---|---|
| practice-rs | 00-brand (tokens, ramps, typography, math, rules) | `front matter`, values `observed`/in-use, contrast computed | Narrow-width and dark-adjacent checks not yet recorded |
| jyas-webapp | 00-brand (zai-light re-key, geometry, components, checklist) | `front matter` + `reviewed reference` checklist (verified live 2026-09-05) | Keep the checklist current across editions |
| zcodereversed | 00-contents + ch-buttons/composer/sidebar-rows/scroll-rail/command/dialogs/icons + 00-brand | chapters `working specimen` (mini-lab), editioned + registry | Human critique pass per chapter → `reviewed reference` |
| t3codereversed | same shape as zcodereversed (the shared mini-lab) | as above | as above |

## What conversion requires (unchanged from the spec)

One complete book slice first — contents header, chapter row, interactive
component, deterministic fixtures, reset, inspectable source, critique
reference — then review the composition before replicating it. The schema-only
runtime is the constraint to resolve, not a reason to lower the bar: until a
chapter mounts its mini-app inline, its honest status is above, and no chapter
may borrow a higher one.
