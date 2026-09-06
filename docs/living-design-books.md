<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
# Living design books

Owner direction, 2026-09-07. This supersedes the v1 flat shelf, still-image
introduction and schema-counter specimen acceptance rules. It describes the
required product, not an already-delivered runtime.

## A book, not a dashboard

A notebook is one row group. Its header opens the book's front matter and
table of contents. Its ordered child rows open chapters; each chapter explains
one component or coherent design decision. A chapter is not its own group.

The opening page establishes the book's purpose, visual identity, inheritance,
edition and reading order. The table of contents uses descriptive chapter
titles, not implementation IDs. Deep links identify book and chapter
independently of their current position. Returning to contents preserves place.

The ambition is an exceptionally well-edited design book: generous specimens,
controlled typography, a consistent page grid, quiet navigation, deliberate
sequence, and precise captions. Polish is not extra card borders, more chrome,
or a dashboard full of tuning switches. Small screens and zoom must retain a
coherent reading order without shrinking the component's interaction targets.

## The specimen is the implementation

Every component chapter opens with a real interactive Dioxus mini-app using
the actual shared component. Demo data wires meaningful behavior, not an
unrelated row renderer standing in for the component. A ribbon must operate a
small editable document. A vault sidebar must search and select fixture
accounts, open details, preserve return context and simulate a fill outcome.

The same component source is available to product implementers. The mini-app
supplies only fixtures, scenario controls and explanatory scaffolding. A
copied lookalike that diverges from the shared component is not a fallback
implementation. Where the component does not yet exist, the chapter must say
so; its first implementation should become the shared source, not a notebook
fork. Proposals remain labeled until accepted.

Images may document provenance, anatomy or historic failures. They cannot
substitute for the component specimen. An external testbed link is useful
support, but not completion of an inline mini-app chapter.

Each specimen supplies:

- Stable book, chapter, specimen and scenario IDs.
- The component source, dependency revision and a runnable example command.
- Deterministic invented fixtures, default state and a complete Reset.
- Real input/state/output behavior, including empty, loading and failure
  scenarios where relevant. Simulated services are explicitly labeled.
- A keyboard path and observable outcome. A callback echo is not acceptance.
- Clear isolation: no real credentials, clipboard writes, external account
  changes or production APIs. Any simulated clipboard is internal demo state.

Markdown remains authored content, not arbitrary executable source. A reviewed
specimen registry binds chapter IDs to compiled mini-apps. Opening a notebook
must not compile or execute code from an untrusted external repository.

## Chapter rhythm

1. **Experience it.** A large working specimen in its intended context; one
   short invitation to perform a meaningful job.
2. **Understand it.** Concise rationale tied to what the reader just did:
   recognition, pointing, focus, hierarchy and return to work.
3. **Compare it.** A live rejected variant using the same fixture and starting
   state. Explain the tradeoff; do not ask the reader to trust an adjective.
4. **Inspect it.** Named tokens, source, scenario selection and state
   inspection, disclosed when useful rather than surrounding the specimen.
5. **Critique it.** A review linked to the exact edition, specimen and scenario.

Switching variants must define whether state is carried or reset; never
silently compare different starting states. Reset must restore the document
or fixtures, selection, search, errors and transient state, not only a counter.

## Human critique becomes a design revision

A critique records the edition/component revision, fixture, viewport/scale,
input method, observed problem, expected result and proposed change. Capture
the specimen state, not private application data. Allow ordinary free-text
commentary; this is a design conversation, not a mandatory long survey.

Review outcomes are proposed, accepted, rebutted or deferred, with rationale.
An accepted critique changes component/example and notebook together; the next
edition links the revision and its verification. A comment does not silently
rewrite the base language. App-brand critiques remain in their app layer;
shared-component reasoning can graduate into the base book.

## Runtime responsibilities

Keep the existing notebook CLI, registration and inheritance records useful.
The current schema-only runtime is an implementation constraint to resolve,
not a reason to weaken the specimen requirement.

The book model and chapter manifest belong to ydesign. Shared component code
belongs to its component package. Dioxus demo composition belongs to a
separately identifiable example target with explicit dependencies; do not
silently link the current schema-only CLI against a new platform stack.
The host owns generic surface mounting and lifecycle. Select the existing
supported surface contract before implementing that bridge; this document
does not invent a wire widget or prescribe an unverified embedding API.

## Conversion and acceptance

First implement one complete book slice: contents header, chapter row,
interactive component, deterministic fixtures, reset, inspectable source and
critique reference. Start with Ribbon, then Complex sidebar. Review the
composition before replicating it across forms, feedback, typography,
iconography and the remaining component inventory.

Maintain a chapter inventory with explicit status: legacy illustration,
working specimen, or reviewed reference implementation. Do not call the
collection converted while chapters remain illustrations. Non-component
front matter may remain prose; component chapters may not.

Accept a converted chapter only after its real specimen completes the task,
keyboard focus is correct, reset is repeatable, narrow/zoomed reading is
usable, and a human can identify the source and attach an actionable critique.
The prior schema-row counters and still diagrams fail this new acceptance bar.

This specification does not claim that mounting, persistence of critiques,
all-component conversion or the row-group navigation has shipped. It does not
authorize unrelated product redesigns or copying a reference book's imagery.
