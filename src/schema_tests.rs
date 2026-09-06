    use super::*;
    #[test]
    fn book_header_opens_contents_and_disclosure_preserves_selection() {
        let mut view = View::default();
        let opening = viewport_view(&view);
        assert_eq!(opening["widgets"][0]["id"], "book-opening");
        assert!(opening["widgets"].as_array().unwrap().iter()
            .any(|w| w["row_action"] == "page_open:ribbons:0"));
        let rail = rail_view(&view);
        assert!(rail["widgets"].as_array().unwrap().iter()
            .any(|w| w["id"] == "book:yggui" && w["depth"] == 0));
        assert!(rail["widgets"].as_array().unwrap().iter()
            .any(|w| w["id"] == "notebook:ribbons" && w["depth"] == 1));
        assert!(view.book_action("book_toggle:yggui"));
        assert_eq!(viewport_view(&view), opening);
        assert!(!rail_view(&view)["widgets"].as_array().unwrap().iter()
            .any(|w| w["depth"] == 1));
        assert!(!view.book_action("book_open:missing"));
        assert!(view.book_action("book_open:yggui"));
        assert!(view.expanded_books.contains("yggui"));
    }

    #[test]
    fn study_changes_visible_state_and_resets_without_external_actions() {
        let mut view = View { selected_book: None, selected_notebook: Some("complex-sidebars".into()), ..View::default() };
        let before = viewport_view(&view);
        assert!(view.study_action("study:open"));
        assert_ne!(before, viewport_view(&view));
        assert!(view.study_action("study:fill"));
        assert_eq!(view.study_actions, 1);
        assert!(view.study_action("study:back"));
        assert!(!view.study_detail);
        assert!(!view.study_action("fill-real-vault"));
        assert!(view.study_action("study:reset"));
        assert_eq!(view.study_actions, 0);
        assert!(view.study_proposed);
    }

