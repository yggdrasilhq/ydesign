//! The notebook shelf — ydesign's content model.
//!
//! Base notebooks are SOURCE-CONTROLLED in this repository and ship with the
//! binary (embedded with `include_str!`), the same doctrine ytop uses: a base
//! notebook is a reading, so it is versioned, reviewable and checkable like
//! code. Agent-composed notebooks are one JSON file per notebook under the
//! user's data dir and never shadow a shipped id.
//!
//! ⚠ LICENCE SPLIT: everything under `notebooks/` (and `docs/`) is
//! documentation and carries CC-BY-SA-4.0 (`LICENSE-CC-BY-SA-4.0`); the Rust
//! source is GPL-3.0-or-later. The split is stated in NOTICE and README, and
//! every notebook file opens with an SPDX doc comment saying which it is.

use serde::{Deserialize, Serialize};

pub const MODE_GUIDE: &str = "guide";
pub const MODE_EXAMPLES: &str = "examples";

/// Notebooks that appear on the shelf in BOTH modes: the roadmap and the
/// start-here page are part of working, not of one mode's reading list.
pub const ALWAYS_VISIBLE: &[&str] = &["start-here", "roadmap"];

/// The catalogue's screenshots, embedded at build time. The notebook
/// markdown references them as `%ASSETS%/name.png`; the schema substitutes a
/// `data:image/png;base64` URL built from these bytes — data URLs render
/// from ANY page origin, which file:// does not (WebKit blocks file
/// subresources from the shell's own origin; the first build of this
/// exhibited as bare caption chips where the screenshots should have been).
pub const ASSETS: &[(&str, &[u8])] = &[
    ("ribbon-study.svg", include_bytes!("../assets/ribbon-study.svg")),
    ("vault-study.svg", include_bytes!("../assets/vault-study.svg")),
    (
        "catalogue-emd-reader.png",
        include_bytes!("../assets/catalogue-emd-reader.png"),
    ),
    (
        "catalogue-forms-rail.png",
        include_bytes!("../assets/catalogue-forms-rail.png"),
    ),
    (
        "catalogue-notebook-page.png",
        include_bytes!("../assets/catalogue-notebook-page.png"),
    ),
];

/// The served markdown: every `%ASSETS%/name.png` becomes an inline data URL.
pub fn resolve_asset_paths(markdown: &str) -> String {
    use base64::Engine as _;
    let mut out = markdown.to_string();
    for (name, bytes) in ASSETS {
        let placeholder = format!("%ASSETS%/{name}");
        if out.contains(&placeholder) {
            let data_url = format!(
                "data:{};base64,{}",
                if name.ends_with(".svg") { "image/svg+xml" } else { "image/png" },
                base64::engine::general_purpose::STANDARD.encode(bytes)
            );
            out = out.replace(&placeholder, &data_url);
        }
    }
    out
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub markdown: String,
    /// Reserved for a future live-block vocabulary. Always empty today; the
    /// `#[serde(default)]` keeps older composed notebooks loadable.
    #[serde(default)]
    pub composed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub title: String,
    pub mode: String,
    pub description: String,
    pub author: String,
    pub created_at_ms: u64,
    pub pages: Vec<Page>,
}

impl Notebook {
    pub fn one_page(mut self) -> Self {
        if self.pages.len() > 1 {
            self.pages.truncate(1);
        }
        self
    }
}

/// The shipped shelf, in reading order. Each entry embeds one markdown source;
/// the file is the editable, diffable, CC-BY-SA truth and the binary carries
/// its bytes.
pub fn base_notebooks() -> Vec<Notebook> {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let shipped: &[(&str, &str, &str, &str, &str)] = &[
        ("foundations", MODE_GUIDE, "Foundations — the visual language", "Color, typography, material and relationships.", include_str!("../notebooks/11-foundations.md")),
        ("ribbons", MODE_GUIDE, "Ribbons — commands in context", "Rejected and proposed anatomy, command grouping and keyboard intent.", include_str!("../notebooks/09-ribbons.md")),
        ("complex-sidebars", MODE_GUIDE, "Complex sidebars — task and identity", "A vault study: identity, progressive disclosure and common action paths.", include_str!("../notebooks/10-complex-sidebars.md")),
        ("inheritance", MODE_GUIDE, "Inheritance — the base chain", "Dioxus, yggui and app notebook ownership.", include_str!("../Inheritance.md")),
        (
            "start-here",
            MODE_GUIDE,
            "Start here — the base language",
            "What the design language is, the layer ladder, and how to work under it.",
            include_str!("../notebooks/00-start-here.md"),
        ),
        (
            "gallery",
            MODE_GUIDE,
            "Component gallery",
            "Every yggui component and schema widget, live, with the one-owner rule each encodes.",
            include_str!("../notebooks/01-component-gallery.md"),
        ),
        (
            "sidebars",
            MODE_GUIDE,
            "Sidebars — the canonical patterns",
            "The Live Sessions anatomy, sidebar partitioning, the row engine, and the status vocabulary.",
            include_str!("../notebooks/02-sidebars.md"),
        ),
        (
            "forms",
            MODE_GUIDE,
            "Forms & settings",
            "Section cards, the one field skin, pinned primary actions, and the short-phrase rule.",
            include_str!("../notebooks/03-forms-and-settings.md"),
        ),
        (
            "motion",
            MODE_GUIDE,
            "Motion & feedback",
            "Toasts and their anchors, the stage curtain, the shared blink clock, and desktop-fast curves.",
            include_str!("../notebooks/04-motion-and-feedback.md"),
        ),
        (
            "emd",
            MODE_GUIDE,
            "emd & notebooks",
            "What emd-renderer is, the component contracts, and the demanded components not built yet.",
            include_str!("../notebooks/05-emd-and-notebooks.md"),
        ),
        (
            "catalogue",
            MODE_GUIDE,
            "The catalogue — the design, exhibited",
            "Real pixels from the running fleet: the patterns, and the choices behind them.",
            include_str!("../notebooks/08-design-catalogue.md"),
        ),
        (
            "examples",
            MODE_EXAMPLES,
            "Worked examples — mini-webapps",
            "The canonical sidebars rebuilt as real schemas you can screenshot and compare against.",
            include_str!("../notebooks/06-worked-examples.md"),
        ),
        (
            "roadmap",
            MODE_GUIDE,
            "Roadmap — demanded components",
            "Components the apps have demanded, each with its forcing consumer and admission gate.",
            include_str!("../notebooks/07-roadmap.md"),
        ),
    ];
    let mut notebooks: Vec<_> = shipped
        .iter()
        .map(|(id, mode, title, description, md)| {
            let body = strip_licence_banner(md);
            Notebook {
                id: (*id).to_string(),
                title: (*title).to_string(),
                mode: (*mode).to_string(),
                description: (*description).to_string(),
                author: "ydesign".to_string(),
                created_at_ms: now_ms,
                pages: vec![Page {
                    id: format!("{id}-page"),
                    title: (*title).to_string(),
                    markdown: body,
                    composed: false,
                }],
            }
            .one_page()
        })
        .collect();
    if let Some(index) = notebooks.iter().position(|nb| nb.id == "start-here") {
        let home = notebooks.remove(index);
        notebooks.insert(0, home);
    }
    notebooks
}

/// `notebooks/*.md` open with a machine-readable licence banner so a reader of
/// the raw file knows the doc licence without opening NOTICE. It is stripped
/// before the page is shown.
fn strip_licence_banner(source: &str) -> String {
    let mut text = source;
    if let Some(rest) = text.strip_prefix("<!--")
        && let Some(end) = rest.find("-->") {
            text = rest[end + 3..].trim_start_matches(['\n', '\r']);
        }
    text.to_string()
}

pub fn get_notebook(id: &str) -> Option<Notebook> {
    base_notebooks().into_iter().chain(crate::projects::notebooks()).find(|nb| nb.id == id)
}

/// The shelf for a mode: base notebooks first (never shadowed), then composed
/// ones from disk, deduped by (mode, title).
pub fn list_notebooks(mode: Option<&str>) -> Vec<Notebook> {
    let mut out = base_notebooks();
    out.extend(crate::projects::notebooks());
    out.retain(|nb| match mode {
        Some(m) => nb.mode == m || ALWAYS_VISIBLE.contains(&nb.id.as_str()),
        None => true,
    });
    out
}

/// True when the viewport should append the LIVE widget appendix after the
/// page's markdown — the "mini-webapp in the notebook" half. Only the pages
/// that exist to exhibit real controls compose; a pure reading page never
/// grows a random control block under it.
pub fn composes_live_widgets(notebook_id: &str) -> bool {
    matches!(notebook_id, "gallery" | "examples" | "ribbons" | "complex-sidebars" | "forms" | "motion" | "emd")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shelf_is_complete_and_ids_are_unique() {
        let ids: Vec<_> = base_notebooks().iter().map(|nb| nb.id.clone()).collect();
        let mut sorted = ids.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(ids.len(), sorted.len(), "notebook ids must be unique");
        for required in [
            "start-here",
            "gallery",
            "sidebars",
            "forms",
            "motion",
            "emd",
            "catalogue",
            "examples",
            "roadmap",
        ] {
            assert!(ids.contains(&required.to_string()), "missing {required}");
        }
    }

    #[test]
    fn every_notebook_has_one_nonempty_page_with_a_title_heading() {
        for nb in base_notebooks() {
            assert_eq!(nb.pages.len(), 1, "{} must be single-page", nb.id);
            let md = &nb.pages[0].markdown;
            assert!(!md.trim().is_empty(), "{} page is empty", nb.id);
            assert!(
                md.trim_start().starts_with('#'),
                "{} page must open with a heading",
                nb.id
            );
            assert!(
                !md.contains("SPDX-License-Identifier"),
                "{} page must have its licence banner stripped",
                nb.id
            );
        }
    }

    #[test]
    fn the_licence_banner_is_stripped_and_body_starts_at_the_heading() {
        let nb = get_notebook("start-here").expect("shipped");
        assert!(nb.pages[0].markdown.starts_with("# "));
    }

    #[test]
    fn guide_mode_shows_design_studies_and_inheritance() {
        let guide = list_notebooks(Some(MODE_GUIDE));
        assert!(guide.iter().all(|nb| nb.mode == MODE_GUIDE));
        for id in ["ribbons", "complex-sidebars", "inheritance"] {
            assert!(guide.iter().any(|nb| nb.id == id));
        }
        let examples = list_notebooks(Some(MODE_EXAMPLES));
        // Examples mode shows only its own notebook plus the always-visible pair.
        assert_eq!(examples.len(), 3);
    }

    #[test]
    fn the_catalogue_references_embedded_assets_and_the_resolver_resolves_them() {
        let nb = get_notebook("catalogue").expect("shipped");
        let md = &nb.pages[0].markdown;
        assert!(md.contains("%ASSETS%/catalogue-forms-rail.png"));
        let resolved = resolve_asset_paths(md);
        assert!(!resolved.contains("%ASSETS%"), "placeholder must be gone");
        assert_eq!(
            resolved.matches("data:image/png;base64,").count(),
            3,
            "every exhibit becomes an inline data URL"
        );
    }

    #[test]
    fn only_exhibition_pages_compose_live_widgets() {
        assert!(composes_live_widgets("gallery"));
        assert!(composes_live_widgets("examples"));
        assert!(!composes_live_widgets("start-here"));
        assert!(!composes_live_widgets("roadmap"));
    }
}
