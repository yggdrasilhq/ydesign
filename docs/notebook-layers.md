<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Notebook layers and project registration

Owner specification, 2026-09-05. Notebooks are the design system for both
people and agents. They must explain choices through large visual examples
and small working interactions, rather than collecting rules alone.

## Where decisions live

`DESIGN.md` is a routing and workflow page. Brand identity, semantic palette,
typography, component examples, imagery and rationale live in `design/*.md`
notebooks and `design/assets/`. Existing design decisions must be moved into
notebooks without losing their rationale; migration is not permission to
redesign an app's brand.

`design/Inheritance.md` names the layer, its parents, their order, and what is
overridden. Default chain: Dioxus components → yggui base → app layer.
Read the complete parent chain. Undefined decisions inherit; explicit local
decisions override. Reject cycles and unresolved conflicts during review.
The base chain lives in ydesign's root `Inheritance.md`. The registry loads
documents; it does not compute CSS or prove the correctness of inheritance.

## Initialize and register

```sh
ydesign init /path/to/worktree --id sample-app
ydesign --config /path/to/projects.json --notebook
ydesign --notebook project/sample-app/00-brand --page 1
```

`init` creates missing `DESIGN.md`, `design/Inheritance.md`,
`design/00-brand.md`, and `design/assets/`. It preserves existing files and
registers the repo in the local config, defaulting to
`$XDG_CONFIG_HOME/ydesign/projects.json` (normally `~/.config/ydesign/`).
Use `--config` to keep an isolated registry for a worktree or review.

```json
{"projects":[{"id":"sample-app","path":"/path/to/repo"}]}
```

Paths are absolute or relative to the config file. Keep machine-specific
registries outside public git. After a worktree is merged and removed, point
the registry at the persistent main checkout. Restart ydesign after editing
the registry or notebook source: it loads a snapshot at startup.

External Markdown files become separately selectable notebooks in the Guide
shelf. IDs are namespaced `project/<id>/<filename-without-extension>`; they
cannot replace the base. Each file needs a `# heading`. Images use
`%ASSETS%/image.png` and resolve inside that project's `design/assets/` as
inline data URLs. PNG, JPEG, WebP and SVG are supported, up to 8 MiB each.
Unavailable projects, duplicate IDs and escaped asset paths fail visibly.
Only register trusted notebook repositories; Markdown is data, never executable
JavaScript. Renderer contracts still govern supported EMD fences.

## A useful notebook

Start with the person's task. Show a legible specimen near its explanation;
show the failure beside or immediately before the correction. Explain why the
change helps recognition, pointing, reading, or returning to work. Include
keyboard and mouse paths, narrow-width and zoom behavior, and failure states.
Label evidence as observed, reconstructed, proposed, or verified.

A miniature interaction must change meaningful state and be resettable. A
button that only echoes “clicked” proves transport, not the design. Today's
host disables EMD panel controls; use supported schema actions for live
specimens and document any missing renderer capability. Never present disabled
controls or a static picture as an interactive component.

## Scope and acceptance

This version covers scaffolding, local multi-project shelves, notebook assets,
design inheritance records, and design teaching. It does not import app code,
generate CSS, execute notebook scripts, change app brands, or repair the shared
component renderer. Future inheritance automation must resolve parent IDs,
cycles and conflict declarations before claiming automatic composition.

Check init preservation and repeatability, namespaced lookup, missing-path
errors, asset boundaries and rendering. Visual verification must include a
real notebook page, an external project page, a meaningful specimen state
change, and a narrow/zoomed reading. Proposed designs remain proposals until
their component implementation receives its own pixel and interaction proof.
