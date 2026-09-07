<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# List specimen — first verification

The Dioxus/WebAssembly build passed fleet CI at integration commit 89751d5.
The root application has 24 passing tests; the specimen model has two passing
tests covering stable fixture identity, navigation and full reset.

The standalone page rendered in the WebKit test engine at 1280 × 900, DPR 1.
Twelve chapter controls were present. The contents heading and ordered-list
container both began at x=192 CSS px. This is the declared ordinal-edge grid;
chapter title and description share the next text column.

Observed interaction results:

- Opening chapter c4 rendered its List views destination and focused Back.
- Back restored all twelve entries and focused chapter-c4.
- Long labels plus the 520px column produced a two-line title (48px height),
  with aligned heading/list edges and no document horizontal overflow.
- Reset returned the book width to 1040px, removed the inspector, restored
  twelve entries and cleared the current chapter.

These actions were driven through DOM controls and their resulting state
read back. They verify component callbacks and programmatic focus return,
not a complete keyboard-only sequence. The narrow-column control does not
prove a 390px browser viewport or 200% browser zoom. Those checks remain open.

The page is still a standalone staging target. It is not yet the embedded
notebook specimen or the shared host's repaired list renderer. Critique is an
unsaved local draft. Inline mounting, durable review integration, rejected
layout comparison and the remaining component mini-apps remain work.
