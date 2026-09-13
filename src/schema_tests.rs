use super::*;

/// Seed the project shelf once for the whole test binary: a scratch repo
/// registered as project `yggui`, the no-shipping law in action - the tests
/// exercise the reader against REGISTERED notebooks, never shipped ones.
fn seed_shelf() {
    use std::sync::Once;
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let scratch = std::env::temp_dir().join(format!(
            "ydesign-schema-tests-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let repo = scratch.join("repo");
        std::fs::create_dir_all(repo.join(".git")).unwrap();
        std::fs::create_dir_all(repo.join("design/notebooks")).unwrap();
        std::fs::write(repo.join("design/Inheritance.md"), "# Inheritance\n").unwrap();
        for (stem, body) in [
            ("00-start-here", "# Start here\n\nThe reading order.\n"),
            ("01-component-gallery", "# Component gallery\n\nThe specimens.\n"),
            ("03-forms-and-settings", "# Forms and settings\n\nShort phrases.\n"),
            ("06-worked-examples", "# Worked examples\n\nReal schemas.\n"),
            ("09-ribbons", "# Ribbons\n\nThe ribbon exercise.\n"),
            ("10-complex-sidebars", "# Complex sidebars\n\nThe vault study.\n"),
        ] {
            std::fs::write(
                repo.join("design/notebooks").join(format!("{stem}.emd")),
                body,
            )
            .unwrap();
        }
        let config = scratch.join("registry.json");
        std::fs::write(
            &config,
            format!(
                "{{\"projects\":[{{\"id\":\"yggui\",\"path\":\"{}\"}}]}}",
                repo.display()
            ),
        )
        .unwrap();
        crate::projects::load(&config).unwrap();
    });
}

#[test]
fn book_header_opens_contents_and_disclosure_preserves_selection() {
    seed_shelf();
    let mut view = View::default();
    let opening = viewport_view(&view);
    assert_eq!(opening["widgets"][0]["id"], "book-opening");
    assert!(
        opening["widgets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["row_action"] == "page_open:project/yggui/09-ribbons:0")
    );
    let rail = rail_view(&view);
    assert!(
        rail["widgets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["id"] == "book:project/yggui" && w["depth"] == 0)
    );
    assert!(
        rail["widgets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["id"] == "notebook:project/yggui/09-ribbons" && w["depth"] == 1)
    );
    assert!(view.book_action("book_toggle:project/yggui"));
    assert_eq!(viewport_view(&view), opening);
    assert!(
        !rail_view(&view)["widgets"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["depth"] == 1)
    );
    assert!(!view.book_action("book_open:missing"));
    assert!(view.book_action("book_open:project/yggui"));
    assert!(view.expanded_books.contains("project/yggui"));
}

#[test]
fn study_changes_visible_state_and_resets_without_external_actions() {
    seed_shelf();
    let mut view = View {
        selected_book: None,
        selected_notebook: Some("project/yggui/10-complex-sidebars".into()),
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
    seed_shelf();
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
    assert!(view.book_action("book_toggle:project/yggui"));
    assert!(view.select_mode(MODE_EXAMPLES));
    assert_eq!(view.expanded_books, View::expanded_all(MODE_EXAMPLES));
    assert!(view.select_mode(MODE_GUIDE));
    assert_eq!(view.expanded_books, View::expanded_all(MODE_GUIDE));
}

#[test]
fn a_saved_reading_place_restores_and_stale_ids_fall_back_to_home() {
    seed_shelf();
    let saved = crate::persist::SavedView {
        mode: MODE_GUIDE.into(),
        selected_book: None,
        selected_notebook: Some("forms".into()),
        selected_page: Some("forms-page".into()),
        expanded_books: vec!["project/yggui".into()],
    };
    let view = View::restore(&saved);
    let (home, home_page) = mode_home(MODE_GUIDE).unwrap();
    assert_eq!(view.selected_notebook.as_deref(), Some(home.as_str()));
    assert_eq!(view.selected_page.as_deref(), Some(home_page.as_str()));
    assert_eq!(view.selected_book, Some(book_id_of(&home)));

    // Stale ids - a removed notebook, a renamed book, a typo in the mode -
    // fall back to the guide home and the still-existing expansions, never
    // a half-broken view.
    let stale = crate::persist::SavedView {
        mode: "no-such-mode".into(),
        selected_book: Some("ghost-book".into()),
        selected_notebook: Some("no-such-notebook".into()),
        selected_page: Some("no-such-page".into()),
        expanded_books: vec!["ghost-book".into(), "project/yggui".into()],
    };
    let view = View::restore(&stale);
    assert_eq!(view.mode, MODE_GUIDE);
    let (home, home_page) = mode_home(MODE_GUIDE).unwrap();
    assert_eq!(view.selected_notebook.as_deref(), Some(home.as_str()));
    assert_eq!(view.selected_page.as_deref(), Some(home_page.as_str()));
    assert_eq!(
        view.expanded_books,
        ["project/yggui".to_string()]
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
    // The examples shelf is empty under the no-shipping law (project books
    // are guide-mode), so the restored home is the honest empty state.
    assert_eq!(view.selected_notebook.as_deref(), None);
    assert!(view.expanded_books.is_empty());
    // But a cold start with NO saved state opens fully populated.
    assert_eq!(
        View::default().expanded_books,
        View::expanded_all(MODE_GUIDE)
    );
}
