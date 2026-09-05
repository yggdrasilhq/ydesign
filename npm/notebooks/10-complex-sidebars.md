<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Complex sidebars — make the next action obvious

Someone opens a vault to sign in, choose another account, copy a field, or
change an entry. They should see the right identity quickly, act without
precision pointing, and return to the page with their place intact.

![Vault study: setup occupying every page versus a task-first account list](%ASSETS%/vault-study.svg)

## What is wrong, and what is known

The rejected study is reconstructed from a reported vault layout and source
inspection on 2026-09-05. A shadow capture confirmed repeated generic globe
bubbles, small metadata, and a long list competing with sync and explanatory
chrome. In that captured state the shadow had no active website context, so
the passkey enrollment card was absent. Do not describe that image as proof
of the card's height on a site. The inspected row measured 264 × 38 CSS px,
with 12 px text and a 15 px SVG icon; its generic icon bubble was 26 px.
This is one measured desktop state, not a universal size across themes.

The source inserts passkey setup before every tab's content. For a site with
no stored passkey, a card explains browser internals and asks for enrollment.
That spends the same vertical space during unrelated jobs, including viewing
or editing an entry. A healthy subsystem should not need a standing tutorial.
An error should explain the consequence and offer a repair at the relevant
point; technical details can be disclosed on demand.

The login row replaces identity with credential type: key for passkey, globe
otherwise. That makes unrelated sites look alike and makes the same site's
identity change when its credential changes. Use the site's favicon in a
stable identity slot, with a letter fallback; show Password/Passkey as a
separate quiet label. A favicon helps recognition but never proves the origin.
Keep the actual site and account text legible alongside it.

## Give each level one job

The header identifies the vault and exposes search and Add. The list starts
with accounts matching the current site, with an explicit “All items” route.
A row has a 24 px identity image, a readable site/name, a secondary account
line, a credential label, and a visible Fill action when applicable. Its body
opens details. These actions have different labels and hit regions; clicking
Copy must not also open the item or fill a page.

Use 36 px action targets and roughly 56–64 px two-line rows as the initial
desktop design target. Increase for touch; test at the user's real scale.
More rows on screen is useful only while a person can distinguish and hit
them. An 18 px symbol may be readable inside a 36 px button; the symbol itself
must not be the whole click target.

Details replace the list in the same rail with a labeled Back action. Back
restores the search, scroll and selected account. Editing is an explicit step;
Save remains reachable at the bottom, errors appear at the relevant fields,
and Cancel returns to the entry. No permanent partitions are repeated inside
every sliding page. Reserve a compact footer for vault-wide lock/sync state.

Passkey enrollment belongs to a site task or entry details. Offer “Set up a
passkey” where it is meaningful. If a page must reopen, say what will happen
and preserve the user's context; do not expose “shim,” “arm,” or browser API
names as the primary explanation. Enrollment never means a passkey has been
created, and a proposed UX must preserve the existing user-presence ceremony.

## Count actions from a stated starting point

| Task, with the vault already open | Proposed mouse path | Keyboard intent |
| --- | --- | --- |
| Fill a matching account | Fill on the matching row: 1 click | Select account, invoke labeled Fill |
| Inspect an account | Row body: 1 click | Arrow to row, Enter |
| Copy password | Row menu then Copy password: 2 clicks | Row menu shortcut, named command |
| Change an account | Row, Edit, Save: 3 clicks excluding typing | Enter, Edit, fields, Save |
| Return to results | Back: 1 click, previous place restored | Escape from details, focus restored |

These are acceptance targets, not measured speedups. Compare against a
recorded current path using the same starting state. Never optimize click
count by removing an intentional confirmation or by making the row's action
ambiguous. Destructive work stays separated from frequent work.

## Check the difficult states

Try two accounts on one site, a missing favicon, a long email, no matches,
a locked vault, stale sync, a failed fill, keyboard-only use, 200% zoom and a
narrow rail. Readability, target size and focus matter for everyone: fatigue,
trackpad imprecision and divided attention are ordinary operating conditions.
Search and per-site matching should reduce choices before typography is shrunk.

The miniature below uses invented entries only. Switch the study, open an
entry, return, or invoke simulated Fill. It records the outcome without
touching a real vault or clipboard. Shared row rendering and actual vault
flows require their own implementation and live proof.
