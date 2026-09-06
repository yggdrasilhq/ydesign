---
name: ydesign
description: Consult or extend the yggui design language through rendered notebooks, app design layers, ribbon and sidebar specimens, and project scaffolding.
---

# Ydesign

The notebooks are the visual authority. Read the project's DESIGN.md routing
page, design/Inheritance.md and relevant local notebooks, then the parent
chain into ydesign's Inheritance.md and base notebooks. See
docs/notebook-layers.md for the registration and inheritance contract.

Keep brand identity, palette, typography and visual rationale in notebooks and
assets. DESIGN.md should explain where to read and how to work. Preserve
existing brand decisions when migrating them. `ydesign init <repo> --id <id>`
creates only missing scaffolding and registers the project; use an isolated
`--config` for worktrees, then register the persistent checkout after landing.

For a component decision, identify the human task and common mouse/keyboard
path first. Read the relevant notebook: ribbons (09), complex sidebars (10),
forms (03), motion (04), or component gallery (01). Show a large, legible
specimen and a meaningful state transition. Explain the rejected pattern and
the reason it fails. Label proposals separately from measured implementation.

- One row group is one book. Its header opens the book's contents; child rows
  are component chapters. Read docs/living-design-books.md before changing the
  book model or authoring a component chapter.
- Lead each component chapter with its actual interactive Dioxus mini-app,
  deterministic demo data, meaningful behavior and Reset. Reuse the real shared
  component and expose source/dependency revision. A screenshot, callback
  counter, substitute schema row or external link is not a completed specimen.
- Keep reading polished and quiet; disclose source, scenario and state tools.
  Human critiques identify the edition, scenario and observed behavior; accepted
  revisions update component and notebook together.

Use the yggui-app-control named shadow for desktop evidence. Record build,
viewport, state, target geometry and faithful pixels. Use invented fixture
data in published examples; private captures stay out of the repo. Do not
claim a disabled EMD control is interactive. Existing schema actions can prove
transport but do not satisfy the Dioxus specimen contract. File the mounting
gap with its owner and label the chapter incomplete until it hosts the component.

For unresolved UX choices, consult the owner's preferred UX advisor via the
fleet advisor protocol. Give the task, current pixels, interaction paths,
alternatives and narrow question. Put accepted reasoning in the notebook,
not only in a board conversation. App-specific content stays in the app layer;
reusable component defects go to their actual renderer owner.
