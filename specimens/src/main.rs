use dioxus::prelude::*;
use ydesign_specimens::{
    fixtures, ribbon_fixture, vault_fixture, Anatomy, ChapterList, CommandKind, FillOutcome,
    ListDensity, RibbonStudy, Study, VaultStudy,
};

fn main() { dioxus::launch(App); }

#[derive(Clone, Copy, PartialEq)]
enum StudyKind {
    List,
    Ribbon,
    Vault,
}

impl StudyKind {
    fn label(self) -> &'static str {
        match self { StudyKind::List => "List views", StudyKind::Ribbon => "Ribbons", StudyKind::Vault => "Complex sidebars" }
    }
    fn eyebrow(self) -> &'static str {
        match self {
            StudyKind::List => "WORKING STUDY 01",
            StudyKind::Ribbon => "WORKING STUDY 02",
            StudyKind::Vault => "WORKING STUDY 03",
        }
    }
    fn title(self) -> &'static str {
        match self {
            StudyKind::List => "A list worth reading.",
            StudyKind::Ribbon => "Commands that belong to the workspace.",
            StudyKind::Vault => "The next action, made obvious.",
        }
    }
    fn lede(self) -> &'static str {
        match self {
            StudyKind::List => "One column. Clear chapter identities. Enough room to read before choosing.",
            StudyKind::Ribbon => "Tabs name tasks, groups gather commands, and Save stays one click when pinned.",
            StudyKind::Vault => "Recognize the identity, act without precision pointing, and return with your place intact.",
        }
    }
}

#[component]
fn App() -> Element {
    let mut study = use_signal(|| StudyKind::List);
    rsx! {
        style { {include_str!("../assets/book.css")} }
        main { class: "book",
            header {
                p { class: "eyebrow", "LIVING DESIGN BOOKS · STAGING TARGET" }
                h1 { "ydesign working studies." }
                p { class: "lede", "Three interactive component studies with deterministic fixtures, a complete reset, and a critique reference. The shared renderers they teach remain their owners' pending changes." }
                nav { class: "study-switcher", aria_label: "Studies",
                    for kind in [StudyKind::List, StudyKind::Ribbon, StudyKind::Vault] {
                        button {
                            class: "study-tab",
                            aria_current: if study() == kind { "page" } else { "false" },
                            onclick: move |_| study.set(kind),
                            "{kind.label()}"
                        }
                    }
                }
            }
            match study() {
                StudyKind::List => rsx! { ListStudyPage {} },
                StudyKind::Ribbon => rsx! { RibbonStudyPage {} },
                StudyKind::Vault => rsx! { VaultStudyPage {} },
            }
        }
    }
}

#[component]
fn Critique(value: String, oninput: EventHandler<String>, review_text: String) -> Element {
    rsx! {
        footer {
            details {
                summary { "Critique this study" }
                label { r#for: "critique", "What helped or obstructed your task?" }
                textarea { id: "critique", value: "{value}", oninput: move |e| oninput.call(e.value()) }
                p { "Draft only: not posted or saved. Copy the review text manually to share it. Reset clears the draft." }
                pre { class: "review", "{review_text}" }
            }
        }
    }
}

#[component]
fn StudyHeader(kind: StudyKind) -> Element {
    rsx! {
        p { class: "eyebrow", "{kind.eyebrow()}" }
        h2 { "{kind.title()}" }
        p { class: "lede", "{kind.lede()}" }
        p { class: "status", "Interactive Dioxus study · invented data · staging target, not the shared renderer" }
    }
}

// ─── Study 01 — list views ──────────────────────────────────────────────────

#[component]
fn ListStudyPage() -> Element {
    let mut study = use_signal(Study::default);
    let mut inspect = use_signal(|| false);
    let state = study.read().clone();
    let selected = fixtures(state.long_labels).into_iter()
        .find(|c| Some(&c.id) == state.selected.as_ref());
    let review = state.review_text();
    rsx! {
        section { class: "reading", aria_label: "List study",
            StudyHeader { kind: StudyKind::List }
            div { class: "tools",
                button { onclick: move |_| inspect.toggle(), aria_expanded: "{inspect}", "Inspect study" }
                button { onclick: move |_| { study.write().reset(); inspect.set(false); }, "Reset" }
            }
            if inspect() {
                aside { class: "inspector", aria_label: "Study controls",
                    label { input { r#type: "checkbox", checked: state.long_labels, onchange: move |e| study.write().long_labels = e.checked() } "Long labels" }
                    label { input { r#type: "checkbox", checked: state.narrow, onchange: move |e| study.write().narrow = e.checked() } "Narrow column" }
                    label { input { r#type: "checkbox", checked: state.compact, onchange: move |e| study.write().compact = e.checked() } "Compact density" }
                    p { "Compare density, not different data. Narrow column is a layout exercise, not browser zoom proof." }
                }
            }
            div { class: "reading", aria_label: "Chapter contents",
                if let Some(chapter) = selected {
                    button {
                        id: "back-to-contents",
                        onmounted: move |event| async move {
                            let _ = event.set_focus(true).await;
                        },
                        onclick: move |_| {
                            let id = study.read().last_opened.clone().unwrap_or_default();
                            study.write().back();
                            spawn(async move {
                                // Wait for the Dioxus render before restoring the actual target.
                                let script = format!("requestAnimationFrame(() => requestAnimationFrame(() => document.getElementById('chapter-{id}')?.focus()));");
                                let _ = document::eval(&script).await;
                            });
                        },
                        "← Contents"
                    }
                    article {
                        h3 { "{chapter.title}" }
                        p { "{chapter.description}" }
                        p { "This destination proves the list's navigation contract. Return to contents to inspect the same entry and fixture state." }
                    }
                } else {
                    h3 { "Contents" }
                    ChapterList {
                        chapters: fixtures(state.long_labels),
                        current: state.last_opened.clone(),
                        density: if state.compact { ListDensity::Compact } else { ListDensity::Editorial },
                        on_open: move |id: String| { study.write().open(&id); },
                    }
                }
            }
        }
        Critique {
            value: state.critique.clone(),
            oninput: move |v: String| study.write().critique = v,
            review_text: review,
        }
    }
}

// ─── Study 02 — ribbons ─────────────────────────────────────────────────────

#[component]
fn RibbonStudyPage() -> Element {
    let mut study = use_signal(RibbonStudy::new);
    let mut inspect = use_signal(|| false);
    let state = study.read().clone();
    let review = state.review_text();
    let rejected = state.variant == Anatomy::Rejected;
    let show_band = state.pinned || state.expanded;
    rsx! {
        section { class: "reading", aria_label: "Ribbon study",
            StudyHeader { kind: StudyKind::Ribbon }
            div { class: "tools",
                button { onclick: move |_| inspect.toggle(), aria_expanded: "{inspect}", "Inspect study" }
                button { onclick: move |_| { study.write().reset(); inspect.set(false); }, "Reset" }
            }
            if inspect() {
                aside { class: "inspector", aria_label: "Study controls",
                    fieldset { class: "radio-row",
                        legend { "Anatomy" }
                        label { input { r#type: "radio", name: "anatomy", checked: !rejected, onchange: move |_| study.write().set_variant(Anatomy::Proposed) } "Proposed: grouped band" }
                        label { input { r#type: "radio", name: "anatomy", checked: rejected, onchange: move |_| study.write().set_variant(Anatomy::Rejected) } "Rejected: floating panel" }
                    }
                    label { input { r#type: "checkbox", checked: state.pinned, onchange: move |e| study.write().set_pinned(e.checked()) } "Pin the ribbon (reserves its space)" }
                    if !state.pinned {
                        button { onclick: move |_| study.write().toggle_overlay(),
                            if state.expanded { "Close temporary panel" } else { "Open temporary panel" }
                        }
                    }
                    p { "Switching anatomy resets to that anatomy's default — the two are never silently compared from different states." }
                }
            }
            div { class: if rejected { "ribbon rejected" } else if state.pinned { "ribbon pinned" } else { "ribbon temporary" },
                                    div { class: "ribbon-tabs", role: "tablist", aria_label: "Ribbon tabs (arrow keys move)",
                    tabindex: "0",
                    onkeydown: move |evt: KeyboardEvent| {
                        match evt.key() {
                            Key::ArrowRight => { ribbon_step(&mut study, 1); }
                            Key::ArrowLeft => { ribbon_step(&mut study, -1); }
                            _ => {}
                        }
                    },
                    for (id, label) in ribbon_tabs() {
                        button {
                            class: "ribbon-tab",
                            role: "tab",
                            aria_selected: if state.active_tab == id { "true" } else { "false" },
                            onclick: move |_| { let _ = study.write().select_tab(id); },
                            "{label}"
                        }
                    }
                }
                if !rejected && !state.pinned {
                    button {
                        class: "command",
                        aria_expanded: "{state.expanded}",
                        aria_controls: "ribbon-band",
                        onclick: move |_| study.write().toggle_overlay(),
                        if state.expanded { "Hide commands" } else { "Show commands" }
                    }
                }
                if show_band {
                    if rejected {
                        div { class: "ribbon-panel rejected-panel", role: "group", aria_label: "Commands (rejected composition)",
                            for group in state.active_tab_groups() {
                                for command in group.commands {
                                    button {
                                        class: if command.kind == CommandKind::Primary { "command primary" } else { "command" },
                                        onclick: move |_| ribbon_command(&mut study, command.id),
                                        "{command.label}"
                                    }
                                }
                            }
                            span { class: "panel-note", "The SAME commands, ungrouped and floating — only the composition differs." }
                        }
                    } else {
                        div { class: "ribbon-band", id: "ribbon-band", role: "group", aria_label: "Grouped commands",
                            for group in state.active_tab_groups() {
                                div { class: "ribbon-group",
                                    div { class: "ribbon-commands",
                                        for command in group.commands {
                                            button {
                                                class: if command.kind == CommandKind::Primary { "command primary" } else if command.kind == CommandKind::Toggle { "command toggle" } else { "command" },
                                                aria_pressed: if state.toggled.iter().any(|t| t == command.id) { "true" } else { "false" },
                                                onclick: move |_| ribbon_command(&mut study, command.id),
                                                "{command.label}"
                                            }
                                        }
                                        if group.label == "Find" {
                                            input {
                                                class: "find-field",
                                                r#type: "search",
                                                placeholder: "Find in document",
                                                aria_label: "Find in document",
                                                value: "{state.find_query}",
                                                oninput: move |e| study.write().set_find_query(e.value()),
                                            }
                                            button {
                                                class: "command",
                                                onclick: move |_| {
                                                    let n = study.read().find_matches();
                                                    study.write().log.push(format!("Find: {n} match(es) in the document."));
                                                },
                                                "Find ({state.find_matches()})"
                                            }
                                            input {
                                                class: "find-field",
                                                r#type: "text",
                                                placeholder: "Replace with",
                                                aria_label: "Replace with",
                                                value: "{state.replace_with}",
                                                oninput: move |e| study.write().set_replace_with(e.value()),
                                            }
                                            button { class: "command", onclick: move |_| { study.write().replace_all(); }, "Replace all" }
                                        }
                                    }
                                    span { class: "ribbon-caption", "{group.label}" }
                                }
                            }
                        }
                    }
                } else {
                    p { class: "panel-note", "The ribbon is collapsed. Its commands are inside the closed panel — try Save and watch the two-click path cost." }
                }
            }
            div { class: "document-area",
                label { r#for: "ribbon-doc", "Document (invented fixture)" }
                textarea {
                    id: "ribbon-doc",
                    rows: "6",
                    value: "{state.document}",
                    oninput: move |e| study.write().edit_document(e.value()),
                }
                p { class: "status",
                    if state.saved { "Saved." } else { "Unsaved changes." }
                }
            }
            ul { class: "log", aria_live: "polite",
                for line in state.log.iter().rev().take(6) {
                    li { "{line}" }
                }
            }
        }
        Critique {
            value: state.critique.clone(),
            oninput: move |v: String| study.write().critique = v,
            review_text: review,
        }
    }
}

/// Arrow keys move between tabs; the roving order is the fixture order.
fn ribbon_step(study: &mut Signal<RibbonStudy>, dir: i32) {
    let tabs = ["home", "review"];
    let current = study.read().active_tab.clone();
    let index = tabs.iter().position(|t| *t == current).unwrap_or(0);
    let next = ((index as i32 + dir).rem_euclid(tabs.len() as i32)) as usize;
    let _ = study.write().select_tab(tabs[next]);
}

fn ribbon_tabs() -> Vec<(&'static str, &'static str)> {
    ribbon_fixture().into_iter().map(|t| (t.id, t.label)).collect()
}

fn ribbon_command(study: &mut Signal<RibbonStudy>, id: &'static str) {
    // Signals are Copy: own a copy for the spawned task so no borrow escapes.
    let mut owned = *study;
    match id {
        "save" => {
            // The save guard must drop before the refusal is logged.
            let result = { study.write().save() };
            if let Err(reason) = result {
                study.write().log.push(reason.into());
            }
            // The focus proof: refocus the document, restore the exact caret
            // position, and report it — an observed result, not a claim.
            spawn(async move {
                let script = "requestAnimationFrame(() => requestAnimationFrame(() => { const el = document.getElementById('ribbon-doc'); if (el) { const pos = el.selectionStart ?? 0; el.focus(); el.setSelectionRange(pos, pos); window.__ribbonCaret = pos; } }));";
                let _ = document::eval(&script).await;
                let pos = document::eval("window.__ribbonCaret ?? -1").await.ok()
                    .and_then(|v| v.as_i64())
                    .unwrap_or(-1);
                if pos >= 0 {
                    owned.write().log.push(format!("Focus is back in the document; caret preserved at {pos}."));
                }
            });
        }
        "spellcheck" => {
            study.write().toggle_command("spellcheck");
            study.write().log.push("Spelling toggled (a toggle command, not a state jump).".into())
        }
        other => study.write().log.push(format!("{other} opened its group workflow (Find and Replace stay together).").into()),
    }
}

// ─── Study 03 — complex sidebars (the vault) ────────────────────────────────

/// Keyboard and pointer Back share ONE restoration path: close details and
/// return focus to the entry the reader came from.
fn vault_back(study: &mut Signal<VaultStudy>) {
    let id = study.read().last_selected.clone().unwrap_or_default();
    study.write().back();
    spawn(async move {
        let script = format!("requestAnimationFrame(() => requestAnimationFrame(() => document.getElementById('vault-row-{id}')?.focus()));");
        let _ = document::eval(&script).await;
    });
}

#[component]
fn VaultStudyPage() -> Element {
    let mut study = use_signal(VaultStudy::default);
    let mut inspect = use_signal(|| false);
    let state = study.read().clone();
    let review = state.review_text();
    let visible = state.visible();
    let visible_is_empty = visible.is_empty();
    let selected = state.selected.as_ref()
        .and_then(|id| vault_fixture().into_iter().find(|a| &a.id == id));
    rsx! {
        section { class: "reading", aria_label: "Vault study",
            StudyHeader { kind: StudyKind::Vault }
            div { class: "tools",
                button { onclick: move |_| inspect.toggle(), aria_expanded: "{inspect}", "Inspect study" }
                button { onclick: move |_| { study.write().reset(); inspect.set(false); }, "Reset" }
            }
            if inspect() {
                aside { class: "inspector", aria_label: "Study controls",
                    label { input { r#type: "checkbox", checked: state.simulate_failure, onchange: move |e| study.write().simulate_failure = e.checked() } "Simulate a failed fill" }
                    fieldset { class: "radio-row",
                        legend { "Current page origin" }
                        for site in ["example.test", "shop.test", "other.test"] {
                            label { input { r#type: "radio", name: "origin", checked: state.current_site == site, onchange: move |_| study.write().change_origin(site) } "{site}" }
                        }
                    }
                    p { "Scenario: two accounts on example.test (the recognition test), a non-matching pair, one long address, favicon and letter-fallback marks. Changing the origin re-evaluates every match. Fill reports its outcome; failure keeps the selection and the page." }
                }
            }
            p { class: "scenario", aria_label: "Current page", "Current page: example.test" }
            if let Some(account) = selected {
                section {
                    class: "vault-details",
                    aria_label: "Account details",
                    onkeydown: move |evt: KeyboardEvent| {
                        if evt.key() == Key::Escape {
                            evt.stop_propagation();
                            vault_back(&mut study);
                        }
                    },
                    button {
                        id: "back-to-vault-list",
                        onclick: move |_| vault_back(&mut study),
                        "← Back to accounts"
                    }
                    p { class: "status", "Escape also returns, with focus restored to the entry you came from." }
                    h3 { "{account.site}" }
                    dl { class: "vault-facts",
                        div { dt { "Account" } dd { "{account.user}" } }
                        div { dt { "Credential" } dd { "{account.credential.label()}" } }
                        div { dt { "Matches this page" } dd { if account.matches(study.read().current_site) { "Yes" } else { "No" } } }
                    }
                    if account.matches(study.read().current_site) {
                        button { class: "command primary", onclick: move |_| { study.write().fill(account.id); }, "Fill on this page" }
                    } else {
                        p { class: "status", "{account.site} does not match this page; Fill is not offered here." }
                    }
                }
            } else {
                div { class: "vault-search",
                    input {
                        r#type: "search",
                        placeholder: "Search accounts",
                        aria_label: "Search accounts",
                        value: "{state.query}",
                        oninput: move |e| study.write().search(e.value()),
                    }
                    if state.needs_all_items_route() {
                        button { class: "command", onclick: move |_| study.write().show_all(), "No matches for this site — show all items" }
                    }
                }
                ul { class: "vault-list", aria_label: "Accounts",
                    for account in visible {
                        li {
                            div { class: "vault-row",
                                button {
                                    id: "vault-row-{account.id}",
                                    class: "vault-open",
                                    title: "Open {account.site} details",
                                    onclick: move |_| { let _ = study.write().open(account.id); },
                                    span {
                                        class: if account.has_favicon { "vault-mark favicon" } else { "vault-mark" },
                                        style: if account.has_favicon { "background:{account.mark_color}" } else { "" },
                                        aria_hidden: "true",
                                        if account.has_favicon { "" } else { "{account.mark()}" }
                                    }
                                    span { class: "vault-copy",
                                        span { class: "vault-site", "{account.site}" }
                                        span { class: "vault-user", "{account.user}" }
                                    }
                                    span { class: "vault-credential", "{account.credential.label()}" }
                                }
                                if account.matches(study.read().current_site) {
                                    button {
                                        class: "command vault-fill",
                                        title: "Fill this account into the page",
                                        onclick: move |_| { study.write().fill(account.id); },
                                        "Fill"
                                    }
                                }
                            }
                        }
                    }
                    if visible_is_empty {
                        li { class: "vault-empty", "No account matches this page. Search, or take the explicit all-items route above — never a blank rail." }
                    }
                }
            }
            if let Some(outcome) = &state.outcome {
                p { class: "outcome", aria_live: "polite",
                    match outcome {
                        FillOutcome::Filled(text) => rsx! { span { class: "outcome-ok", "✓ {text}" } },
                        FillOutcome::Failed(text) => rsx! { span { class: "outcome-fail", "✗ {text}" } },
                    }
                }
            }
        }
        Critique {
            value: state.critique.clone(),
            oninput: move |v: String| study.write().critique = v,
            review_text: review,
        }
    }
}
