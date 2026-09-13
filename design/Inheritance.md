<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Inheritance

Layer: **ydesign, the reader app**. This repository carries no design
language of its own beyond the reading surfaces it renders.

Parent: yggui (L1, libyggterm) — whose `design/notebooks/*.emd` are the base
design language this app exists to exhibit. Transitive: Dioxus (L0).

The specimen demo styling (specimens/assets/book.css) inherits the yggui
token names; anything the reader renders is governed by the layer that owns
the component being exhibited.
