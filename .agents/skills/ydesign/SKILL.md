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

Use the yggui-app-control named shadow for desktop evidence. Record build,
viewport, state, target geometry and faithful pixels. Use invented fixture
data in published examples; private captures stay out of the repo. Do not
claim a disabled EMD control is interactive: use schema actions supported by
the host, or file the renderer gap with its owner.

For unresolved UX choices, consult the owner's preferred UX advisor via the
fleet advisor protocol. Give the task, current pixels, interaction paths,
alternatives and narrow question. Put accepted reasoning in the notebook,
not only in a board conversation. App-specific content stays in the app layer;
reusable component defects go to their actual renderer owner.
