<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Inheritance

Layer: yggui base design notebooks, published by ydesign.

Parent: Dioxus components (component and event substrate). The yggui layer
adds semantic theme tokens, typography, row anatomy, focus, ribbon, rail and
feedback patterns. Dioxus alone does not prescribe the yggui visual identity.

An app inheriting yggui inherits this chain transitively. App notebooks may
override named visual decisions, with rationale and specimens; structural
accessibility, state ownership and truthful feedback remain requirements.
For multiple parents, list them in precedence order and explicitly resolve
overlapping decisions. A cycle or unresolved overlap is invalid.

This is design inheritance, not a dependency manager: consult each project's
manifest for the actual Dioxus and yggui versions. No code is imported by
registering a notebook.
