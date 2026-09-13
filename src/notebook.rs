//! The notebook shelf, ydesign's content model.
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

/// THE NO-SHIPPING LAW (owner, 2026-09-13): the ydesign app is the READER.
/// It ships no base notebooks. The component vocabulary lives in the
/// platform that owns the components (libyggterm: design/notebooks/*.emd),
/// chrome books live in yggterm, app books live in the app repos, and the
/// registry (~/.yggterm/config/ydesign/projects.json) assembles the shelf.
/// The ydesign skill onboards a host by registering those repositories.
pub fn base_notebooks() -> Vec<Notebook> {
    Vec::new()
}

pub fn get_notebook(id: &str) -> Option<Notebook> {
    base_notebooks()
        .into_iter()
        .chain(crate::projects::notebooks())
        .find(|nb| nb.id == id)
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

/// Books group the legacy chapter records without changing their deep-link IDs.
pub struct Book {
    pub id: String,
    pub title: String,
    pub chapters: Vec<Notebook>,
}

pub fn books(mode: Option<&str>) -> Vec<Book> {
    let mut books = vec![Book {
        id: "yggui".into(),
        title: "Yggui, the design language".into(),
        chapters: Vec::new(),
    }];
    for chapter in list_notebooks(mode) {
        let project = chapter
            .id
            .strip_prefix("project/")
            .and_then(|rest| rest.split_once('/'))
            .map(|(id, _)| id.to_owned());
        let id = project
            .as_ref()
            .map(|p| format!("project/{p}"))
            .unwrap_or_else(|| "yggui".into());
        let index = if let Some(index) = books.iter().position(|b| b.id == id) {
            index
        } else {
            books.push(Book {
                id,
                title: project.unwrap(),
                chapters: Vec::new(),
            });
            books.len() - 1
        };
        books[index].chapters.push(chapter);
    }
    books
}

/// True when the viewport should append the LIVE widget appendix after the
/// page's markdown, the "mini-webapp in the notebook" half. Only the pages
/// that exist to exhibit real controls compose; a pure reading page never
/// grows a random control block under it.
pub fn composes_live_widgets(notebook_id: &str) -> bool {
    const LIVE_SUFFIXES: &[&str] = &[
        "01-component-gallery",
        "03-forms-and-settings",
        "04-motion-and-feedback",
        "05-emd-and-notebooks",
        "06-worked-examples",
        "09-ribbons",
        "10-complex-sidebars",
    ];
    LIVE_SUFFIXES
        .iter()
        .any(|suffix| notebook_id.ends_with(suffix))
}
