//! The widget schema — what yggterm paints in both Viewport and Rail surfaces.
//!
//! Rail: the notebook shelf, in reading order, with the mode switch in the
//! titlebar. Viewport: the open page as a `markdown` widget, and — for the
//! exhibition pages — a COMPOSED appendix of real widgets below the prose, so
//! a reader (human or agent) meets the components as painted controls, not
//! only as text about them.

use crate::notebook::{self, Notebook};
use serde_json::{json, Value};

pub const MODE_GUIDE: &str = notebook::MODE_GUIDE;
pub const MODE_EXAMPLES: &str = notebook::MODE_EXAMPLES;

#[derive(Debug, Clone)]
pub struct View {
    pub mode: String,
    pub selected_notebook: Option<String>,
    pub selected_page: Option<String>,
    pub notice: Option<String>,
}

impl Default for View {
    fn default() -> Self {
        Self {
            mode: MODE_GUIDE.to_string(),
            // ydesign opens on Start here — the language itself is the home
            // page, and there is no nameless view you reach by having
            // selected nothing.
            selected_notebook: Some("start-here".to_string()),
            selected_page: Some("start-here-page".to_string()),
            notice: None,
        }
    }
}

impl View {
    pub fn select_mode(&mut self, mode: &str) -> bool {
        if !matches!(mode, MODE_GUIDE | MODE_EXAMPLES) {
            return false;
        }
        self.mode = mode.to_string();
        let home = if mode == MODE_EXAMPLES {
            "examples"
        } else {
            "start-here"
        };
        self.selected_notebook = Some(home.to_string());
        self.selected_page = Some(format!("{home}-page"));
        self.notice = None;
        true
    }
}

fn section(text: impl Into<String>, card: bool) -> Value {
    json!({"kind": "section", "text": text.into(), "card": card})
}

fn titlebar_switch_spec(active_mode: &str) -> Value {
    json!({
        "active": active_mode,
        "action": "mode",
        "segments": [
            {"id": MODE_GUIDE, "label": "Guide", "title": "The base design language, one notebook per concern"},
            {"id": MODE_EXAMPLES, "label": "Examples", "title": "Canonical surfaces rebuilt as live schemas to compare against"},
        ]
    })
}

// ─── RAIL (the notebook shelf) ────────────────────────────────────────────────

pub fn rail_view(view: &View) -> Value {
    let mut widgets = Vec::new();

    if let Some(notice) = &view.notice {
        widgets.push(json!({"kind": "label", "text": notice, "muted": true}));
    }

    widgets.push(section("Notebooks", false));
    for nb in notebook::list_notebooks(Some(&view.mode)) {
        let selected = view.selected_notebook.as_deref() == Some(&nb.id);
        widgets.push(json!({
            "kind": "list-row",
            "id": format!("notebook:{}", nb.id),
            "title": nb.title,
            "selected": selected,
            "row_action": format!("page_open:{}:0", nb.id),
        }));
    }

    json!({
        "title": "Ydesign",
        "titlebar_switch": titlebar_switch_spec(&view.mode),
        "widgets": widgets,
        "footer": [json!({
            "kind": "label",
            "text": "Base design language · shipped notebooks v0.1",
            "muted": true
        })]
    })
}

// ─── VIEWPORT (the open page) ─────────────────────────────────────────────────

pub fn viewport_view(view: &View) -> Value {
    let mut widgets = Vec::new();

    if let Some(nb_id) = view.selected_notebook.clone()
        && let Some(nb) = notebook::get_notebook(&nb_id) {
            let page = nb
                .pages
                .iter()
                .find(|p| Some(&p.id) == view.selected_page.as_ref())
                .or_else(|| nb.pages.first())
                .cloned();
            if let Some(page) = page {
                widgets.push(json!({
                    "kind": "markdown",
                    "id": format!("book_page:{}", page.id),
                    "source": page.markdown,
                }));
                // ── The exhibition half ─────────────────────────────────────
                // A design language is argued from pixels, so the pages that
                // exist to exhibit components append the REAL widgets below
                // the prose. Screenshot the page; the controls in it are the
                // host's own, painted by the same code every app inherits.
                if notebook::composes_live_widgets(&nb.id) {
                    for widget in exhibition_widgets(&nb, view) {
                        widgets.push(widget);
                    }
                }
                return json!({
                    "title": format!("{} — {}", nb.title, page.title),
                    "titlebar_switch": titlebar_switch_spec(&view.mode),
                    "widgets": widgets,
                    "footer": [json!({
                        "kind": "label",
                        "text": "ydesign · the base design language · notebooks are CC-BY-SA-4.0",
                        "muted": true
                    })]
                });
            }
        }

    // No selection: the empty state names the shelf instead of painting a
    // blank sheet. Reaching it in practice is a bug — the view always opens
    // with a page selected — but the surface must still say something true.
    widgets.push(json!({
        "kind": "markdown",
        "id": "empty",
        "source": "# No notebook open\n\nPick one from the shelf in the sidebar.".to_string(),
    }));
    json!({
        "title": "Ydesign",
        "titlebar_switch": titlebar_switch_spec(&view.mode),
        "widgets": widgets,
    })
}

/// The live appendix for an exhibition page: real painted widgets standing in
/// for the patterns the page's prose describes. Each block is prefixed by a
/// plain section header so the specimen is labelled inside the page flow.
fn exhibition_widgets(nb: &Notebook, view: &View) -> Vec<Value> {
    let mut widgets = Vec::new();
    if nb.id == "gallery" {
        widgets.push(section("Live specimens — controls as the host paints them", false));

        widgets.push(section("A section card is a form's home (card: true)", true));
        widgets.push(json!({
            "kind": "toggle",
            "id": "specimen_autohide",
            "action": "demo",
            "label": "Auto-hide titlebar",
            "value": false,
        }));
        widgets.push(json!({
            "kind": "toggle",
            "id": "specimen_mirror",
            "action": "demo",
            "label": "Mirror chrome",
            "value": false,
        }));

        widgets.push(section("Segmented control — one look for every mode switch", false));
        widgets.push(json!({
            "kind": "tabs",
            "id": "specimen_tabs",
            "action": "demo_tab",
            "active": "one",
            "tabs": [
                {"id": "one", "label": "Segment"},
                {"id": "two", "label": "Segment"},
                {"id": "three", "label": "Segment"},
            ],
        }));

        widgets.push(section("Rows — the shared row engine, Rail density", false));
        for (idx, (title, status, selected)) in [
            ("Saved note — durable", "durable", false),
            ("Draft note — transient", "transient", true),
            ("Brand-new note — empty slot", "", false),
        ]
        .into_iter()
        .enumerate()
        {
            widgets.push(json!({
                "kind": "list-row",
                "id": format!("specimen_row:{idx}"),
                "title": title,
                "status": status,
                "selected": selected,
                "row_action": "demo",
            }));
        }
    }

    if nb.id == "examples" {
        widgets.push(section(
            "Specimen 1 — the Live Sessions anatomy, rebuilt",
            false,
        ));
        widgets.push(json!({
            "kind": "search-box",
            "id": "specimen_live_filter",
            "action": "demo",
            "value": "",
            "placeholder": "Filter sessions…",
        }));
        for (idx, (title, status, badge)) in [
            ("9.4 Continue the novel outline", "durable", ""),
            ("11.0 Orchestrator handoff", "durable", "32"),
            ("Read the memory doors", "transient", ""),
        ]
        .into_iter()
        .enumerate()
        {
            let mut row = json!({
                "kind": "list-row",
                "id": format!("live_row:{idx}"),
                "title": title,
                "status": status,
                "row_action": "demo",
            });
            if !badge.is_empty() {
                row["badge"] = json!(badge);
            }
            widgets.push(row);
        }

        widgets.push(section(
            "Specimen 2 — the partitioned sidebar (top partition ≤ 30%)",
            false,
        ));
        widgets.push(section("Knobs — the small top partition", true));
        widgets.push(json!({
            "kind": "toggle",
            "id": "specimen_wrap",
            "action": "demo",
            "label": "Soft wrap",
            "value": true,
        }));
        widgets.push(section("FILES — the majority partition", false));
        for (idx, (name, status)) in [
            ("triage-board.md", "durable"),
            ("meeting-notes.md", "transient"),
            ("untitled.md", ""),
        ]
        .into_iter()
        .enumerate()
        {
            widgets.push(json!({
                "kind": "list-row",
                "id": format!("file_row:{idx}"),
                "title": name,
                "status": status,
                "row_action": "demo",
            }));
        }
        widgets.push(json!({
            "kind": "label",
            "text": "Status line: 3 files · 1,204 words — the footer is the third partition",
            "muted": true,
        }));
    }

    let _ = view;
    widgets
}
