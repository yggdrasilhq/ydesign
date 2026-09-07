use dioxus::prelude::*;
use ydesign_specimens::{ChapterList, ListDensity, Study, fixtures};

fn main() { dioxus::launch(App); }

#[component]
fn App() -> Element {
    let mut study = use_signal(Study::default);
    let mut inspect = use_signal(|| false);
    let state = study.read().clone();
    let selected = fixtures(state.long_labels).into_iter()
        .find(|c| Some(&c.id) == state.selected.as_ref());
    rsx! {
        style { {include_str!("../assets/book.css")} }
        main { class: if state.narrow { "book narrow" } else { "book" },
            header {
                p { class: "eyebrow", "YGGUI / WORKING STUDY 01" }
                h1 { "A list worth reading." }
                p { class: "lede", "One column. Clear chapter identities. Enough room to read before choosing." }
                p { class: "status", "Interactive Dioxus proposal · invented data · not yet embedded in the notebook host" }
                div { class: "tools",
                    button { onclick: move |_| inspect.toggle(), aria_expanded: "{inspect}", "Inspect study" }
                    button { onclick: move |_| { study.write().reset(); inspect.set(false); }, "Reset" }
                }
            }
            if inspect() {
                aside { class: "inspector", aria_label: "Study controls",
                    label { input { r#type: "checkbox", checked: state.long_labels, onchange: move |e| study.write().long_labels = e.checked() } "Long labels" }
                    label { input { r#type: "checkbox", checked: state.narrow, onchange: move |e| study.write().narrow = e.checked() } "Narrow column" }
                    label { input { r#type: "checkbox", checked: state.compact, onchange: move |e| study.write().compact = e.checked() } "Compact density" }
                    p { "Compare density, not different data. Narrow column is a layout exercise, not browser zoom proof." }
                    details {
                        summary { "Reusable component source" }
                        pre { code { {include_str!("lib.rs")} } }
                    }
                }
            }
            section { class: "reading", aria_label: "Chapter contents",
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
                        h2 { "{chapter.title}" }
                        p { "{chapter.description}" }
                        p { "This destination proves the list's navigation contract. Return to contents to inspect the same entry and fixture state." }
                    }
                } else {
                    h2 { "Contents" }
                    ChapterList {
                        chapters: fixtures(state.long_labels),
                        current: state.last_opened.clone(),
                        density: if state.compact { ListDensity::Compact } else { ListDensity::Editorial },
                        on_open: move |id: String| { study.write().open(&id); },
                    }
                }
            }
            footer {
                details {
                    summary { "Critique this study" }
                    label { r#for: "critique", "What helped or obstructed your task?" }
                    textarea { id: "critique", value: "{state.critique}", oninput: move |e| study.write().critique = e.value() }
                    p { "Draft only: not posted or saved. Copy the review text manually to share it. Reset clears the draft." }
                    pre { class: "review", "{state.review_text()}" }
                }
            }
        }
    }
}
