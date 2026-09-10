use super::*;
#[test]
fn book_header_opens_contents_and_disclosure_preserves_selection() {
    let mut view = View::default();
    let opening = viewport_view(&view);
    assert_eq!(opening["widgets"][0]["id"], "book-opening");
    assert!(
        opening["widgets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["row_action"] == "page_open:ribbons:0")
    );
    let rail = rail_view(&view);
    assert!(
        rail["widgets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["id"] == "book:yggui" && w["depth"] == 0)
    );
    assert!(
        rail["widgets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["id"] == "notebook:ribbons" && w["depth"] == 1)
    );
    assert!(view.book_action("book_toggle:yggui"));
    assert_eq!(viewport_view(&view), opening);
    assert!(
        !rail_view(&view)["widgets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["depth"] == 1)
    );
    assert!(!view.book_action("book_open:missing"));
    assert!(view.book_action("book_open:yggui"));
    assert!(view.expanded_books.contains("yggui"));
}

#[test]
fn study_changes_visible_state_and_resets_without_external_actions() {
    let mut view = View {
        selected_book: None,
        selected_notebook: Some("complex-sidebars".into()),
        ..View::default()
    };
    let before = viewport_view(&view);
    assert!(view.study_action("study:open", ""));
    assert_ne!(before, viewport_view(&view));
    assert!(view.study_action("study:fill", ""));
    assert_eq!(view.study_actions, 1);
    assert!(view.study_action("study:back", ""));
    assert!(!view.study_detail);
    assert!(!view.study_action("fill-real-vault", "test"));
    assert!(view.study_action("study:reset", ""));
    assert_eq!(view.study_actions, 0);
    assert!(view.study_proposed);
}

#[test]
fn the_shelf_opens_fully_populated_and_select_mode_repopulates() {
    let mut view = View::default();
    assert_eq!(view.expanded_books, View::expanded_all(&view.mode));
    let chapters = notebook::list_notebooks(Some(&view.mode)).len();
    let rail = rail_view(&view);
    let rows = rail["widgets"].as_array().unwrap();
    assert_eq!(
        rows.iter().filter(|w| w["depth"] == 0).count(),
        View::expanded_all(&view.mode).len()
    );
    assert_eq!(rows.iter().filter(|w| w["depth"] == 1).count(), chapters);
    // A mode switch is a fresh reading: the whole shelf populates again even
    // after the reader collapsed everything.
    assert!(view.book_action("book_toggle:yggui"));
    assert!(view.select_mode(MODE_EXAMPLES));
    assert_eq!(view.expanded_books, View::expanded_all(MODE_EXAMPLES));
    assert!(view.select_mode(MODE_GUIDE));
    assert_eq!(view.expanded_books, View::expanded_all(MODE_GUIDE));
}

#[test]
fn a_saved_reading_place_restores_and_stale_ids_fall_back_to_home() {
    let saved = crate::persist::SavedView {
        mode: MODE_GUIDE.into(),
        selected_book: None,
        selected_notebook: Some("forms".into()),
        selected_page: Some("forms-page".into()),
        expanded_books: vec!["yggui".into()],
    };
    let view = View::restore(&saved);
    assert_eq!(view.selected_notebook.as_deref(), Some("forms"));
    assert_eq!(view.selected_page.as_deref(), Some("forms-page"));
    assert_eq!(view.selected_book, None);

    // Stale ids - a removed notebook, a renamed book, a typo in the mode -
    // fall back to the guide home and the still-existing expansions, never
    // a half-broken view.
    let stale = crate::persist::SavedView {
        mode: "no-such-mode".into(),
        selected_book: Some("ghost-book".into()),
        selected_notebook: Some("no-such-notebook".into()),
        selected_page: Some("no-such-page".into()),
        expanded_books: vec!["ghost-book".into(), "yggui".into()],
    };
    let view = View::restore(&stale);
    assert_eq!(view.mode, MODE_GUIDE);
    assert_eq!(view.selected_notebook.as_deref(), Some("start-here"));
    assert_eq!(view.selected_page.as_deref(), Some("start-here-page"));
    assert_eq!(
        view.expanded_books,
        ["yggui".to_string()]
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>()
    );

    // An explicitly empty saved expansion set is the reader's own
    // all-collapsed shelf, restore is faithful, not re-expanding.
    let examples = crate::persist::SavedView {
        mode: MODE_EXAMPLES.into(),
        selected_book: None,
        selected_notebook: None,
        selected_page: None,
        expanded_books: vec![],
    };
    let view = View::restore(&examples);
    assert_eq!(view.selected_notebook.as_deref(), Some("examples"));
    assert!(view.expanded_books.is_empty());
    // But a cold start with NO saved state opens fully populated.
    assert_eq!(
        View::default().expanded_books,
        View::expanded_all(MODE_GUIDE)
    );
}
