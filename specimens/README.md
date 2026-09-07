# List view working study

This crate is the living design books' staging target: three interactive
component studies (list views, ribbons, complex sidebars) with deterministic
fixtures, complete resets and a critique reference. The registry binding
chapter IDs to studies is specimens.json. None are mounted inside the
notebook host yet.

`src/lib.rs` exports the proposed reusable `ChapterList`. It receives chapters,
current identity, density and an open callback; it owns neither fixture data
nor page width. `src/main.rs` supplies the demonstration and critique draft.
The component is a candidate shared implementation, not an already-extracted
yggui component. GPL-3.0-or-later, like the repository's other code.

From this directory, `dx serve --web` opens the development target.
Fleet integration uses the repository's build script through ygg-ci.

Try a chapter, return, enable long labels and a narrow column, then repeat.
Reset clears every fixture choice, navigation state and critique draft.
The critique text is local unsaved state, not a posted comment. No production
services, credentials, clipboard writes or remote data are involved.

Acceptance still needs browser/keyboard and responsive pixel evidence. A
narrow column switch is not a substitute for a real viewport and zoom test.
