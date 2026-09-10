//! Reusable proposed list, ribbon and vault components. Demo state is kept
//! outside the components.
//!
//! These are the living-books conversion studies for the Ribbons and Complex
//! sidebars books: real interactive Dioxus mini-apps with deterministic
//! fixtures, a complete reset and a critique reference. They are staging
//! targets, the shared renderers they teach (ribbon renderer, vault row
//! renderer) remain their owners' pending changes.
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ListRow {
    pub id: String,
    /// Display-ready ordinal, e.g. "01".
    pub ordinal: String,
    pub title: String,
    pub description: String,
}

#[component]
pub fn ListTable(
    rows: Vec<ListRow>,
    on_open: EventHandler<String>,
    #[props(default)] current: Option<String>,
) -> Element {
    rsx! {
        ol { class: "list-table",
            for row in rows.iter() {
                li { key: "{row.id}", class: "list-table__item",
                    button {
                        class: "list-table__open",
                        r#type: "button",
                        aria_current: if current.as_ref() == Some(&row.id) { "page" } else { "false" },
                        onclick: {
                            let id = row.id.clone();
                            move |_| on_open.call(id.clone())
                        },
                        span { class: "list-table__ordinal", aria_hidden: "true", "{row.ordinal}" }
                        span { class: "list-table__copy",
                            span { class: "list-table__heading",
                                span { class: "list-table__title", "{row.title}" }
                                if current.as_ref() == Some(&row.id) {
                                    span { class: "list-table__current", "You are here" }
                                }
                            }
                            if !row.description.is_empty() {
                                span { class: "list-table__description", "{row.description}" }
                            }
                        }
                        // One native keyboard target; the chevron is its visible control.
                        span { class: "list-table__trailing", aria_hidden: "true",
                            span { class: "list-table__chevron",
                                svg {
                                    view_box: "0 0 16 16",
                                    fill: "none",
                                    path {
                                        d: "M6 3.5 10.5 8 6 12.5",
                                        stroke: "currentColor",
                                        stroke_width: "1.5",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                    }
                                }
                            }
                        }
                    }
                    // Future independent actions belong beside the row button,
                    // in this item, never nested inside the navigation button.
                }
            }
        }
    }
}

pub fn fixtures(long: bool) -> Vec<ListRow> {
    let titles = [
        "Start with the person", "Color and material", "Typography",
        "List views", "Commands in context", "Forms", "Feedback",
        "Navigation", "Identity", "Motion", "Inheritance", "Review",
    ];
    titles.iter().enumerate().map(|(i, title)| ListRow {
        id: format!("c{}", i + 1),
        ordinal: format!("{:02}", i + 1),
        title: if long && i == 3 {
            "List views that remain readable when chapter titles need more than one line".into()
        } else { (*title).into() },
        description: if i == 7 { String::new() } else if long && i == 3 {
            "The same chapter identity, a deliberately longer description, and enough content to test wrapping without shrinking the text or hiding the words that distinguish this entry.".into()
        } else { format!("A working study of {}.", title.to_lowercase()) },
    }).collect()
}

#[derive(Default, Clone, PartialEq)]
pub struct Study {
    pub selected: Option<String>,
    pub last_opened: Option<String>,
    pub long_labels: bool,
    pub narrow: bool,
    pub critique: String,
}

impl Study {
    pub fn open(&mut self, id: &str) -> bool {
        if !fixtures(self.long_labels).iter().any(|c| c.id == id) { return false; }
        self.selected = Some(id.into());
        self.last_opened = Some(id.into());
        true
    }
    pub fn back(&mut self) { self.selected = None; }
    pub fn reset(&mut self) { *self = Self::default(); }
    pub fn review_text(&self) -> String {
        format!("Book: ydesign\nSpecimen: list-views/v1-proposal\nScenario: long={}, narrow={}\nChapter: {}\nObservation: {}",
            self.long_labels, self.narrow,
            self.last_opened.as_deref().unwrap_or("contents"), self.critique)
    }
}

// ─── Ribbon, commands that belong to the workspace ─────────────────────────

/// The two compositions the Ribbons book compares. Switching resets to the
/// variant's default state; nothing silent is carried across.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Anatomy {
    #[default]
    Proposed,
    Rejected,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CommandKind {
    #[default]
    Ordinary,
    Primary,
    Toggle,
}

#[derive(Clone, PartialEq)]
pub struct RibbonCommand {
    pub id: &'static str,
    pub label: &'static str,
    pub kind: CommandKind,
    /// A disabled command is shown, never silent: the chip reads as real but
    /// unavailable (the mock's Resolve).
    pub disabled: bool,
}

#[derive(Clone, PartialEq)]
pub struct RibbonGroup {
    pub label: &'static str,
    pub commands: Vec<RibbonCommand>,
    /// The find group carries the observable find/replace workflow.
    pub find: bool,
}

#[derive(Clone, PartialEq)]
pub struct RibbonTab {
    pub id: &'static str,
    pub label: &'static str,
    pub groups: Vec<RibbonGroup>,
}

/// Deterministic fixture. The proposed anatomy groups by the person's task
/// (document · find), one primary command only. The rejected anatomy renders
/// the same commands as a sparse floating panel.
pub fn ribbon_fixture() -> Vec<RibbonTab> {
    vec![
        RibbonTab {
            id: "home",
            label: "Home",
            groups: vec![
                RibbonGroup { label: "Find", find: true, commands: vec![
                    RibbonCommand { id: "find", label: "Find", kind: CommandKind::Ordinary, disabled: false },
                    RibbonCommand { id: "replace", label: "Replace", kind: CommandKind::Ordinary, disabled: false },
                ]},
                RibbonGroup { label: "Document", find: false, commands: vec![
                    RibbonCommand { id: "save", label: "Save", kind: CommandKind::Primary, disabled: false },
                ]},
            ],
        },
        RibbonTab {
            id: "review",
            label: "Review",
            groups: vec![
                RibbonGroup { label: "Proofing", find: false, commands: vec![
                    RibbonCommand { id: "spellcheck", label: "Spelling", kind: CommandKind::Toggle, disabled: false },
                    RibbonCommand { id: "resolve", label: "Resolve", kind: CommandKind::Ordinary, disabled: true },
                ]},
            ],
        },
    ]
}

#[derive(Clone, PartialEq)]
pub struct RibbonStudy {
    pub variant: Anatomy,
    pub pinned: bool,
    /// The temporary overlay of a collapsed ribbon; meaningful only when
    /// unpinned. Choosing Save while it is closed is the two-click path.
    pub expanded: bool,
    pub active_tab: String,
    pub document: String,
    pub saved: bool,
    /// Real on/off state for Toggle commands; aria_pressed renders it.
    pub toggled: Vec<String>,
    /// The find query survives tab changes in the proposed anatomy, a query
    /// that disappears on tab change is a reject condition in the book.
    pub find_query: String,
    pub replace_with: String,
    pub log: Vec<String>,
    pub critique: String,
}

impl RibbonStudy {
    pub fn document_fixture() -> &'static str {
        "Quarterly notes (invented)\n\nThe fixture text exists so Save has a real\njob: type, save, and watch the caret stay put."
    }

    pub fn new() -> Self {
        Self {
            variant: Anatomy::Proposed,
            pinned: true,
            expanded: true,
            active_tab: "home".into(),
            document: Self::document_fixture().into(),
            saved: true,
            toggled: Vec::new(),
            find_query: String::new(),
            replace_with: String::new(),
            log: vec!["Pinned ribbon, Home tab, document saved.".into()],
            critique: String::new(),
        }
    }

    pub fn active_tab_groups(&self) -> Vec<RibbonGroup> {
        ribbon_fixture()
            .into_iter()
            .find(|t| t.id == self.active_tab)
            .map(|t| t.groups)
            .unwrap_or_default()
    }

    /// Switching variants resets to that anatomy's default and says so,
    /// never silently compare different starting states.
    pub fn set_variant(&mut self, variant: Anatomy) {
        if self.variant == variant { return; }
        *self = Self::new();
        self.variant = variant;
        if variant == Anatomy::Rejected {
            self.pinned = false;
            self.expanded = false;
            self.log = vec!["Rejected anatomy: sparse floating panel, collapsed by default.".into()];
        }
    }

    pub fn set_pinned(&mut self, pinned: bool) {
        self.pinned = pinned;
        // Pinned reserves its own space; collapsed opens only temporarily.
        self.expanded = pinned;
        self.log.push(if pinned {
            "Pinned: the band reserves space above the document.".into()
        } else {
            "Collapsed: the band is closed; opening it is temporary.".into()
        });
    }

    /// Toggle commands carry real state; the design renders ON as an accent
    /// tint with an inset rule, OFF as the routine command.
    pub fn toggle_command(&mut self, id: &str) {
        if let Some(pos) = self.toggled.iter().position(|t| t == id) {
            self.toggled.remove(pos);
        } else {
            self.toggled.push(id.to_string());
        }
    }

    pub fn toggle_overlay(&mut self) {
        if self.pinned { return; }
        self.expanded = !self.expanded;
        self.log.push(if self.expanded {
            "Panel opened over the document (temporary overlay).".into()
        } else {
            "Panel closed; focus returns to the editor.".into()
        });
    }

    pub fn select_tab(&mut self, id: &str) -> bool {
        if ribbon_fixture().iter().all(|t| t.id != id) { return false; }
        if self.active_tab == id { return true; }
        self.active_tab = id.into();
        self.log.push(if self.find_query.is_empty() {
            format!("Tab → {id}.")
        } else {
            format!("Tab → {id}; the find query “{}” is carried, not dropped.", self.find_query)
        });
        true
    }

    pub fn edit_document(&mut self, text: String) {
        if text != self.document {
            self.document = text;
            self.saved = false;
        }
    }

    pub fn set_find_query(&mut self, q: String) {
        self.find_query = q;
    }

    pub fn set_replace_with(&mut self, q: String) {
        self.replace_with = q;
    }

    /// Find reports an observable result, the number of live matches in the
    /// document, not a button that opens another workflow.
    pub fn find_matches(&self) -> usize {
        let q = self.find_query.as_str();
        if q.is_empty() { return 0; }
        self.document.matches(q).count()
    }

    /// Replace does the editing job: every occurrence is replaced in the
    /// document (which becomes unsaved) and the count is reported.
    pub fn replace_all(&mut self) -> Option<usize> {
        let q = self.find_query.clone();
        if q.is_empty() { return None; }
        let count = self.find_matches();
        if count == 0 { return None; }
        self.document = self.document.replace(&q, &self.replace_with);
        self.saved = false;
        self.log.push(format!(
            "Replaced {count} occurrence(s) of “{q}”; the document is unsaved."
        ));
        Some(count)
    }

    /// Save. Returns Err(reason) when the anatomy makes it a two-click path,
    /// the study refuses and names it, the way the review table says a design
    /// must not silently widen.
    pub fn save(&mut self) -> Result<(), &'static str> {
        if !self.pinned && !self.expanded {
            return Err("Save is inside the closed panel, expand first (two clicks), or pin the ribbon.");
        }
        self.saved = true;
        self.log.push(format!(
            "Saved ({} click path); focus returns to the document.",
            if self.pinned { "one" } else { "overlay" }
        ));
        Ok(())
    }

    pub fn reset(&mut self) { *self = Self::new(); }

    pub fn review_text(&self) -> String {
        format!("Book: ydesign\nSpecimen: ribbons/v1-study\nScenario: variant={:?}, pinned={}, tab={}\nFixture: {}-char invented document\nObservation: {}",
            self.variant, self.pinned, self.active_tab,
            self.document.chars().count(), self.critique)
    }
}

impl Default for RibbonStudy {
    fn default() -> Self { Self::new() }
}

// ─── Vault, a complex sidebar that makes the next action obvious ───────────

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Credential {
    #[default]
    Password,
    Passkey,
}

impl Credential {
    pub fn label(self) -> &'static str {
        match self { Credential::Password => "Password", Credential::Passkey => "Passkey" }
    }
}

#[derive(Clone, PartialEq)]
pub struct VaultAccount {
    pub id: &'static str,
    pub site: &'static str,
    pub user: &'static str,
    pub credential: Credential,
    /// Invented site imagery: two accounts carry a stable favicon square,
    /// two demonstrate the missing-favicon path. Identity is never the
    /// credential icon standing in for the site.
    pub has_favicon: bool,
    pub mark_color: &'static str,
}

impl VaultAccount {
    /// The letter fallback is the identity slot's answer to a missing
    /// favicon: stable, never a credential icon masquerading as site identity.
    pub fn mark(&self) -> char {
        self.site.chars().next().unwrap_or('?').to_ascii_uppercase()
    }
    /// Derived against the CURRENT page, so an origin change re-evaluates
    /// every match instead of silently reusing a stale one.
    pub fn matches(&self, current_site: &str) -> bool {
        self.site == current_site
    }
}

/// Deterministic fixture per the Complex sidebars walkthrough: two accounts
/// on one site (the recognition test), a non-matching pair, one long address,
/// and both favicon states (present + letter fallback).
pub fn vault_fixture() -> Vec<VaultAccount> {
    vec![
        VaultAccount { id: "personal", site: "example.test", user: "reader@example.test", credential: Credential::Passkey, has_favicon: true, mark_color: "#216d81" },
        VaultAccount { id: "work", site: "example.test", user: "w.svenson@example.test", credential: Credential::Password, has_favicon: true, mark_color: "#b3556d" },
        VaultAccount { id: "archive", site: "other.test", user: "very.long.mailbox.name.for.the.wrap-test@archive.other.test", credential: Credential::Password, has_favicon: false, mark_color: "#dcebe9" },
        VaultAccount { id: "shop", site: "shop.test", user: "orders@example.test", credential: Credential::Passkey, has_favicon: false, mark_color: "#dcebe9" },
    ]
}

/// Outcome of a simulated fill. The request being sent is not completion,
/// the study reports a result, and failure keeps every bit of context.
#[derive(Clone, Debug, PartialEq)]
pub enum FillOutcome {
    Filled(String),
    Failed(String),
}

#[derive(Clone, PartialEq)]
pub struct VaultStudy {
    pub query: String,
    pub selected: Option<String>,
    pub last_selected: Option<String>,
    pub show_all: bool,
    pub simulate_failure: bool,
    pub outcome: Option<FillOutcome>,
    /// The page the person is on. Changing it re-evaluates every match;
    /// a stale match is never silently applied to a new destination.
    pub current_site: &'static str,
    pub critique: String,
}

impl Default for VaultStudy {
    fn default() -> Self {
        Self {
            query: String::new(),
            selected: None,
            last_selected: None,
            show_all: false,
            simulate_failure: false,
            outcome: None,
            current_site: "example.test",
            critique: String::new(),
        }
    }
}

impl VaultStudy {
    /// Visible rows: accounts matching the current site first; the query
    /// narrows by site or user. An empty result is a state with a next action
    /// (All items), never a blank rail.
    pub fn visible(&self) -> Vec<VaultAccount> {
        let q = self.query.trim().to_lowercase();
        let all = vault_fixture();
        if q.is_empty() {
            return if self.show_all {
                all
            } else {
                all.into_iter().filter(|a| a.matches(self.current_site)).collect()
            };
        }
        let hit: Vec<VaultAccount> = all.iter()
            .filter(|a| a.site.to_lowercase().contains(&q) || a.user.to_lowercase().contains(&q))
            .cloned()
            .collect();
        if hit.is_empty() && self.show_all {
            all
        } else {
            hit
        }
    }

    /// The explicit escape route shown when the narrowed list is empty.
    pub fn needs_all_items_route(&self) -> bool {
        let q = self.query.trim().to_lowercase();
        !q.is_empty() && vault_fixture().iter()
            .all(|a| !a.site.to_lowercase().contains(&q) && !a.user.to_lowercase().contains(&q))
    }

    pub fn search(&mut self, q: String) {
        self.query = q;
        self.outcome = None;
    }

    pub fn show_all(&mut self) {
        self.show_all = true;
        self.outcome = None;
    }

    /// Change the page's origin (walkthrough step 5): Fill must re-evaluate,
    /// never silently apply an old match to a new destination.
    pub fn change_origin(&mut self, site: &'static str) {
        if self.current_site == site { return; }
        self.current_site = site;
        self.outcome = Some(FillOutcome::Failed(format!(
            "The page changed to {site}; every match and Fill target was re-evaluated."
        )));
    }

    pub fn open(&mut self, id: &str) -> bool {
        if self.visible().iter().all(|a| a.id != id) { return false; }
        self.selected = Some(id.into());
        self.last_selected = Some(id.into());
        true
    }

    /// Back from details: the query, the narrowed list and the return target
    /// survive, only the details view closes.
    pub fn back(&mut self) { self.selected = None; }

    /// Fill is explicit and reports its outcome; failure keeps the selection
    /// and the page context (never returns to an empty list).
    pub fn fill(&mut self, id: &str) -> Option<FillOutcome> {
        let account = vault_fixture().into_iter().find(|a| a.id == id)?;
        if !account.matches(self.current_site) {
            self.outcome = Some(FillOutcome::Failed(format!(
                "{} does not match this page; Fill stays unavailable.",
                account.site
            )));
            return self.outcome.clone();
        }
        self.selected = Some(id.into());
        self.last_selected = Some(id.into());
        self.outcome = Some(if self.simulate_failure {
            FillOutcome::Failed("The page did not accept the fill; the entry is kept and the page is unchanged.".into())
        } else {
            FillOutcome::Filled(format!("Filled {} into {}.", account.user, account.site))
        });
        self.outcome.clone()
    }

    pub fn reset(&mut self) { *self = Self::default(); }

    pub fn review_text(&self) -> String {
        format!("Book: ydesign\nSpecimen: complex-sidebars/v1-study\nScenario: query={:?}, show_all={}, simulate_failure={}\nAccount: {}\nObservation: {}",
            self.query, self.show_all, self.simulate_failure,
            self.last_selected.as_deref().unwrap_or("none"), self.critique)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_preserves_fixture_and_reset_restores_every_field() {
        let mut state = Study { long_labels: true, narrow: true, ..Study::default() };
        assert!(!state.open("untrusted"));
        assert!(state.open("c4"));
        state.back();
        assert!(state.selected.is_none());
        assert_eq!(state.last_opened.as_deref(), Some("c4"));
        assert!(state.long_labels && state.narrow);
        state.critique = "Title wraps, focus must return here.".into();
        assert!(state.review_text().contains("Chapter: c4"));
        state.reset();
        assert!(state == Study::default());
    }

    #[test]
    fn fixture_ids_survive_long_label_variant() {
        assert_eq!(fixtures(false).iter().map(|c| &c.id).collect::<Vec<_>>(),
            fixtures(true).iter().map(|c| &c.id).collect::<Vec<_>>());
        assert_eq!(fixtures(false).len(), 12);
        assert!(fixtures(false).iter().any(|c| c.description.is_empty()));
    }

    #[test]
    fn save_is_one_click_when_pinned_and_refused_when_the_panel_is_closed() {
        let mut pinned = RibbonStudy::new();
        assert!(pinned.pinned);
        pinned.edit_document("typed more".into());
        assert!(!pinned.saved);
        assert!(pinned.save().is_ok());
        assert!(pinned.saved);
        assert!(pinned.log.last().unwrap().contains("one click"));

        let mut collapsed = RibbonStudy::new();
        collapsed.set_variant(Anatomy::Rejected);
        assert!(!collapsed.pinned && !collapsed.expanded);
        let refused = collapsed.save().unwrap_err();
        assert!(refused.contains("two clicks"));
        assert!(collapsed.saved, "a refused save must not mark the document unsaved");
        collapsed.toggle_overlay();
        assert!(collapsed.expanded);
        assert!(collapsed.save().is_ok());
    }

    #[test]
    fn switching_variants_resets_to_that_default_and_says_so() {
        let mut s = RibbonStudy::new();
        s.set_find_query("spacing".into());
        assert!(s.select_tab("review"));
        s.set_variant(Anatomy::Rejected);
        assert_eq!(s.variant, Anatomy::Rejected);
        assert!(!s.pinned && !s.expanded);
        assert!(s.find_query.is_empty());
        assert_eq!(s.active_tab, "home");
        assert!(s.log[0].contains("Rejected anatomy"));
    }

    #[test]
    fn the_find_query_survives_a_tab_change() {
        let mut s = RibbonStudy::new();
        s.set_find_query("tracking".into());
        assert!(s.select_tab("review"));
        assert_eq!(s.find_query, "tracking");
        assert!(s.log.last().unwrap().contains("carried"));
        assert!(!s.select_tab("no-such-tab"));
    }

    #[test]
    fn find_reports_matches_and_replace_edits_the_document() {
        let mut s = RibbonStudy::new();
        assert_eq!(s.find_matches(), 0);
        s.set_find_query("fixture".into());
        let before = s.document.matches("fixture").count();
        assert!(before >= 1);
        assert_eq!(s.find_matches(), before);
        s.set_replace_with("study".into());
        assert_eq!(s.replace_all(), Some(before));
        assert!(!s.document.contains("fixture"));
        assert!(!s.saved);
        assert!(s.log.last().unwrap().contains("Replaced"));
    }

    #[test]
    fn editing_marks_the_document_unsaved_and_reset_restores_the_fixture() {
        let mut s = RibbonStudy::new();
        s.edit_document(RibbonStudy::document_fixture().into());
        assert!(s.saved, "typing the identical text is not a modification");
        s.edit_document("changed".into());
        assert!(!s.saved);
        s.reset();
        assert!(s.saved && s.document == RibbonStudy::document_fixture());
    }

    #[test]
    fn vault_details_back_preserves_search_and_return_target() {
        let mut v = VaultStudy::default();
        v.search("example".into());
        assert!(v.open("work"));
        v.back();
        assert!(v.selected.is_none());
        assert_eq!(v.last_selected.as_deref(), Some("work"));
        assert_eq!(v.query, "example");
    }

    #[test]
    fn vault_fill_reports_outcome_and_failure_keeps_context() {
        let mut v = VaultStudy::default();
        v.open("work");
        let ok = v.fill("work").unwrap();
        assert!(matches!(ok, FillOutcome::Filled(_)));
        v.simulate_failure = true;
        let failed = v.fill("work").unwrap();
        assert!(matches!(failed, FillOutcome::Failed(_)));
        assert_eq!(v.selected.as_deref(), Some("work"), "failure keeps the selection");
        assert!(!v.visible().is_empty(), "failure never empties the rail");
    }

    #[test]
    fn an_origin_change_reevaluates_every_match() {
        let mut v = VaultStudy::default();
        v.open("work");
        v.change_origin("shop.test");
        assert_eq!(v.current_site, "shop.test");
        assert!(v.visible().iter().all(|a| a.matches("shop.test")));
        let stale = v.fill("work").unwrap();
        assert!(matches!(stale, FillOutcome::Failed(_)), "a stale match must not fill");
        assert_eq!(v.selected.as_deref(), Some("work"), "context survives the origin change");
    }

    #[test]
    fn vault_non_matching_site_cannot_fill_and_empty_search_offers_all_items() {
        let mut v = VaultStudy::default();
        assert_eq!(v.fill("archive"),
            Some(FillOutcome::Failed("other.test does not match this page; Fill stays unavailable.".into())));
        v.search("nothing-matches-this".into());
        assert!(vault_fixture().iter().any(|a| !a.has_favicon), "the missing-favicon path stays in the fixture");
        assert!(v.visible().is_empty());
        assert!(v.needs_all_items_route());
        v.show_all();
        assert_eq!(v.visible().len(), vault_fixture().len());
    }

    #[test]
    fn vault_fixture_covers_the_walkthrough_and_reset_restores_every_field() {
        let f = vault_fixture();
        assert_eq!(f.iter().filter(|a| a.matches("example.test")).count(), 2, "two accounts on one site");
        assert!(f.iter().any(|a| a.user.chars().count() > 30), "a long address for the wrap test");
        assert!(f.iter().all(|a| !a.user.is_empty()));
        let mut v = VaultStudy::default();
        v.search("shop".into()); v.open("shop"); v.fill("shop"); v.critique = "note".into();
        v.reset();
        assert!(v == VaultStudy::default());
    }
}

// ─── Live sessions, the most-watched rail, honest at rest ──────────────────

/// Durability: green = survives the app, blue = lives only while it does,
/// empty slot = nothing to say. Distinct from the live/idle status dot.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Durability {
    #[default]
    Survives,
    Transient,
    None,
}

impl Durability {
    pub fn class(self) -> &'static str {
        match self { Durability::Survives => "survives", Durability::Transient => "transient", Durability::None => "none" }
    }
    pub fn label(self) -> &'static str {
        match self {
            Durability::Survives => "survives the app",
            Durability::Transient => "lives only while the app does",
            Durability::None => "nothing to say",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct SessionEntry {
    pub id: &'static str,
    pub title: &'static str,
    pub live: bool,
    pub durability: Durability,
    pub minutes: usize,
}

#[derive(Clone, PartialEq)]
pub struct SessionGroup {
    pub hash: &'static str,
    pub count: usize,
    pub rows: Vec<SessionEntry>,
}

/// Deterministic fixture per the Worked-examples doctrine: one group header
/// with a trailing count, and rows covering live/idle, all three durability
/// states, and a title long enough to force ellipsis.
pub fn sessions_fixture() -> Vec<SessionGroup> {
    vec![SessionGroup {
        hash: "#local",
        count: 3,
        rows: vec![
            SessionEntry { id: "probe", title: "trace-fixing, ytrace probe sweep on the attach path", live: true, durability: Durability::Survives, minutes: 2 },
            SessionEntry { id: "icons", title: "practice L3 icons", live: true, durability: Durability::Transient, minutes: 14 },
            SessionEntry { id: "archive", title: "a very long session title that must ellipsize to make room for the verbs and never push the trailing edge around", live: false, durability: Durability::None, minutes: 183 },
        ],
    }]
}

#[derive(Clone, PartialEq, Default)]
pub struct SessionsStudy {
    pub selected: Option<String>,
    pub killed: Vec<String>,
}

impl SessionsStudy {
    pub fn select(&mut self, id: &str) { self.selected = Some(id.into()); }
    /// Kill is a real action with a real outcome; the row leaves the rail and
    /// the group count follows.
    pub fn kill(&mut self, id: &str) -> bool {
        if self.killed.iter().any(|k| k == id) { return false; }
        self.killed.push(id.into());
        if self.selected.as_deref() == Some(id) { self.selected = None; }
        true
    }
    pub fn visible(&self) -> Vec<SessionGroup> {
        sessions_fixture().into_iter().map(|mut g| {
            g.rows.retain(|r| !self.killed.iter().any(|k| k == r.id));
            g.count = g.rows.len();
            g
        }).filter(|g| !g.rows.is_empty()).collect()
    }
    pub fn review_text(&self) -> String {
        let visible = self.visible();
        let rows: usize = visible.iter().map(|g| g.rows.len()).sum();
        format!(
            "selected: {:?}; killed: {:?}; groups: {}; rows visible: {}",
            self.selected, self.killed, visible.len(), rows
        )
    }
}

/// One rail of live sessions. The status spine is continuous through every
/// row; the title track owns the row at rest; actions exist only on
/// hover/selected/focus-within; the count rides the group header's edge.
#[component]
pub fn SessionList(
    groups: Vec<SessionGroup>,
    selected: Option<String>,
    on_select: EventHandler<String>,
    on_kill: EventHandler<String>,
) -> Element {
    rsx! {
        ul { class: "sess-list", aria_label: "Live sessions",
            for group in groups.iter() {
                li { class: "sess-group-header", key: "{group.hash}",
                span { class: "sess-hash", "#{group.hash.trim_start_matches('#')}" }
                span { class: "sess-count", "{group.count}" }
            }
            for row in group.rows.iter() {
                li {
                    key: "{row.id}",
                    class: "sess-row",
                    aria_selected: if selected.as_deref() == Some(row.id) { "true" } else { "false" },
                    tabindex: "0",
                    onclick: {
                        let id = row.id;
                        move |_| on_select.call(id.to_string())
                    },
                    span {
                        class: if row.live { "sess-status live" } else { "sess-status idle" },
                        aria_label: if row.live { "live" } else { "idle" },
                    }
                    span { class: "sess-title", "{row.title}" }
                    span {
                        class: "sess-durability {row.durability.class()}",
                        aria_label: "{row.durability.label()}",
                    }
                    span { class: "sess-time", "{row.minutes}m" }
                    span { class: "sess-actions",
                        button {
                            class: "sess-act",
                            aria_label: "Kill session",
                            title: "Kill session",
                            onclick: {
                                let id = row.id;
                                move |e: Event<MouseData>| {
                                    e.stop_propagation();
                                    on_kill.call(id.to_string());
                                }
                            },
                            "×"
                        }
                        }
                    }
                }
            }
        }
        }
    }

// ─── RibbonView, the approved ribbon v2 (consult 0008, mock-first) ─────────

/// The ribbon component proper: a tab strip of visible chips over one
/// contained command band. Pinned shows the band always; temporary collapses
/// it behind a visible Show-commands affordance. Toggle state and the
/// find/replace workflow live in the model, never in the component.
#[component]
pub fn RibbonView(
    tabs: Vec<RibbonTab>,
    active: String,
    pinned: bool,
    expanded: bool,
    toggled: Vec<String>,
    find_value: String,
    replace_value: String,
    find_matches: usize,
    on_tab: EventHandler<String>,
    on_command: EventHandler<String>,
    on_find: EventHandler<String>,
    on_replace: EventHandler<String>,
    on_toggle_pin: EventHandler<()>,
    on_toggle_expand: EventHandler<()>,
) -> Element {
    let visible = pinned || expanded;
    let chevron = if visible { "up" } else { "down" };
    let show_label = if visible { "Hide commands" } else { "Show commands" };
    let active_tab = tabs.iter().find(|tab| tab.id == active);
    rsx! {
        div { class: if visible { "ribbon" } else { "ribbon is-collapsed" },
            div { class: "ribbon-tabs",
                div { class: "ribbon-tab-list", role: "tablist", aria_label: "Ribbon tasks",
                    for tab in &tabs {
                        button {
                            key: "{tab.id}", class: "ribbon-tab", r#type: "button", role: "tab",
                            aria_selected: if tab.id == active { "true" } else { "false" },
                            onclick: {
                                let id = tab.id.to_string();
                                move |_| on_tab.call(id.clone())
                            },
                            "{tab.label}"
                        }
                    }
                }
                div { class: "ribbon-mode-controls",
                    if pinned {
                        button { class: "unpin-control", r#type: "button",
                            onclick: move |_| on_toggle_pin.call(()), "Unpin" }
                    } else {
                        if visible {
                            button { class: "pin-control", r#type: "button",
                                onclick: move |_| on_toggle_pin.call(()), "Pin" }
                        }
                        button { class: "show-commands", r#type: "button",
                            aria_expanded: if visible { "true" } else { "false" },
                            onclick: move |_| on_toggle_expand.call(()),
                            CommandIcon { name: chevron }
                            "{show_label}"
                        }
                    }
                }
            }
            if visible {
                div { class: "ribbon-band", role: "region", aria_label: "Commands",
                    if let Some(tab) = active_tab {
                        for group in &tab.groups {
                            div {
                                key: "{group.label}",
                                class: if group.commands.iter().any(|c| c.kind == CommandKind::Primary) {
                                    "ribbon-group has-primary"
                                } else {
                                    "ribbon-group"
                                },
                                role: "group", "aria-label": "{group.label}",
                                div { class: "ribbon-commands",
                                    for command in &group.commands {
                                        RibbonCommandButton {
                                            key: "{command.id}",
                                            command_id: command.id,
                                            label: command.label,
                                            kind: command.kind,
                                            disabled: command.disabled,
                                            pressed: toggled.iter().any(|t| t == command.id),
                                            on_command,
                                        }
                                    }
                                    if group.find {
                                        label { class: "find-field",
                                            CommandIcon { name: "find" }
                                            input {
                                                r#type: "search",
                                                placeholder: "Find in document",
                                                aria_label: "Find in document",
                                                value: "{find_value}",
                                                oninput: move |e| on_find.call(e.value()),
                                            }
                                        }
                                        button {
                                            class: "command", r#type: "button",
                                            onclick: move |_| on_command.call("find-now".into()),
                                            "Find ({find_matches})"
                                        }
                                        input {
                                            class: "replace-field",
                                            r#type: "text",
                                            placeholder: "Replace with",
                                            aria_label: "Replace with",
                                            value: "{replace_value}",
                                            oninput: move |e| on_replace.call(e.value()),
                                        }
                                        button {
                                            class: "command", r#type: "button",
                                            onclick: move |_| on_command.call("replace-all".into()),
                                            "Replace all"
                                        }
                                    }
                                }
                                div { class: "ribbon-caption", "{group.label}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RibbonCommandButton(
    command_id: &'static str,
    label: &'static str,
    kind: CommandKind,
    disabled: bool,
    pressed: bool,
    on_command: EventHandler<String>,
) -> Element {
    let class = match kind {
        CommandKind::Primary => "command primary",
        CommandKind::Toggle => "command toggle",
        CommandKind::Ordinary => "command",
    };
    rsx! {
        button {
            class, r#type: "button",
            disabled,
            aria_pressed: if kind == CommandKind::Toggle { if pressed { "true" } else { "false" } } else { "" },
            onclick: move |_| on_command.call(command_id.to_string()),
            if kind == CommandKind::Toggle {
                span { class: "toggle-switch", aria_hidden: "true",
                    span { class: "toggle-thumb" }
                }
            } else {
                CommandIcon { name: label }
            }
            "{label}"
        }
    }
}

/// Inline line-icon registry (the mock's icons). Unknown commands keep a
/// text-only chip; the icon source notebook owns the family.
#[component]
fn CommandIcon(name: &'static str) -> Element {
    let path = match name {
        "open" => "M3 6h6l2 3h10v11H3z M3 6V4h6l2 2",
        "find" => "M10 3a7 7 0 1 0 0 14 7 7 0 0 0 0-14 M15 15l6 6",
        "comment" => "M21 11a9 9 0 0 1-9 9c-2 0-3-.4-4-1L2 21l2-6a9 9 0 1 1 17-4z",
        "save" => "M3 3h14l4 4v14H3z M7 3v7h10V3 M7 21v-8h10v8",
        "down" => "M6 9l6 6 6-6",
        "up" => "M6 15l6-6 6 6",
        _ => "",
    };
    rsx! {
        if !path.is_empty() {
            svg { class: "command-icon", view_box: "0 0 24 24",
                path { d: path }
            }
        }
    }
}
