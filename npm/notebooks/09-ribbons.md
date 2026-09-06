<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Ribbons — commands that belong to the workspace

A ribbon helps someone recognize a command without remembering its name.
Tabs name tasks; groups gather related commands; familiar commands stay in
predictable places. The document remains the main event.

![Ribbon study: detached sparse toolbar and integrated grouped ribbon](%ASSETS%/ribbon-study.svg)

## What the rejected example teaches

The upper study reconstructs the reported editor ribbon from the supplied
2026-09-05 screenshot using invented content. It is a diagram, not a capture
of a repaired component. Its useful ideas are the short tab names, visible
Open/Save commands, and group captions. Preserve those.

The dark tab strip breaks the workspace's continuous background. The command
panel is inset from both edges, heavily rounded and heavily shadowed: it reads
as a dialog hovering over the first document lines. Four small commands occupy
a panel almost the width of the document; a distant command-search button
adds pointer travel. Two saturated buttons compete for attention. Tiny group
captions do not compensate for a weak grouping structure.

The spreadsheet reference supplied with the report shows a more useful
hierarchy: task tabs, compact groups, separators, labels close to commands,
and a continuous command band. Learn that anatomy; do not reproduce its full
command count in an editor with fewer jobs.

## The desired composition

Tabs use the same shell background as the start page. Selected state is a
quiet underline or connected surface edge, plus readable text; no dark slab
appears just because the content app changes. Commands use a restrained
surface, aligned to the document edges with a shallow separator. Group
commands by the user's task, not by their implementation module.

Use one prominent command only where there is a clear primary task. Routine
Save does not need to compete with command search. Use a consistent line-icon
family with text labels; mixed emoji are unstable in size, baseline and color.
Keep an icon's drawing smaller than its hit area. Start with 32–36 px pointer
targets in dense desktop chrome; touch layouts need larger targets and spacing.
These are design targets, to be verified at actual scale, not measurements of
the current component.

An expanded, pinned ribbon reserves its own space above the document. A
collapsed ribbon may open temporarily over it, anchored flush below the tabs
with restrained elevation. Never confuse these modes: the current renderer
implements the temporary overlay. The pinned composition shown here is a
proposal that needs a shared renderer change.

## Follow the hand and keyboard

From a collapsed state: click Home, then Save — two clicks. From a pinned
state: Save — one click. A keyboard shortcut keeps the document focus and
requires neither click. Command search complements recognition; it does not
replace visible commands for new users.

Tab/Shift+Tab must reach the strip and command groups with visible focus.
Arrow keys move between tabs; Enter/Space opens the selected group. Escape
closes the temporary panel and returns focus to its tab. Choosing a document
command returns focus to the editor when the command needs no dialog. Verify
these behaviors in the implementation; a screenshot cannot establish them.

At narrow widths, keep task tabs discoverable, then collapse lower-priority
groups into labeled menus. Do not shrink labels or targets until they are hard
to read. Test long translated labels, disabled Save, keyboard focus, 200% zoom,
and repeated commands with the panel pinned and collapsed.

## Review a ribbon by doing a job

Use an invented document with unsaved text. Keep the window size, theme and
document identical across comparisons. Begin with the ribbon collapsed; then
repeat with it pinned. Count actions separately for the two starting states.

| Job | What the person needs | Reject the design if |
| --- | --- | --- |
| Save, then keep typing | Recognize Save, see confirmation, regain the caret | Save steals focus permanently, closes the wrong surface, or hides the first line |
| Find a word, then replace it | Related commands stay together; the search state stays visible | Find and Replace move to unrelated groups or the query disappears on tab change |
| Discover an unfamiliar command | Read task labels, scan groups, use command search if needed | Every command is an unlabeled icon or search is the only discoverable route |
| Repeat a formatting action | Predictable location with the panel pinned | Every click collapses the panel despite the chosen pinned mode |

Do not fill empty ribbon space with commands merely to resemble an office
suite. The reference teaches grouping and continuity, not maximum density.
An editor with four useful commands may need a compact command band; it does
not need a nearly empty floating dialog. Equally, removing all labels to make
the band look cleaner makes an unfamiliar command harder to recognize.

For each review, record three separate verdicts: composition, task completion,
and keyboard/focus. A beautiful screenshot can pass the first and fail the
other two. Capture the tab focus ring and the document's first line as well as
the resting ribbon. A keyboard test passes only when the intended command ran
and focus returned to the correct place, not when a key event was accepted.

## Exercise limits

The live specimen below switches between the rejected and proposed anatomy
and records an example command. It teaches the state and grouping choices;
the figure is an explanatory composition, not the shared ribbon renderer.
Reset restores the starting state. Acceptance of the actual ribbon still
requires a faithful capture and keyboard test in its consumer.
