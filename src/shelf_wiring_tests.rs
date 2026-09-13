//! The no-shipping law: the ydesign app is the READER (owner, 2026-09-13).
//! It ships no base notebooks; the books live in the repos that own the
//! components and surfaces, and the registry assembles the shelf.

#[test]
fn the_binary_ships_no_base_notebooks() {
    assert!(
        crate::notebook::base_notebooks().is_empty(),
        "ydesign ships no base notebooks; they live in the owning repos \
         (libyggterm: the component books, yggterm: the chrome books, apps: \
         their own). The registry assembles the shelf."
    );
}

#[test]
fn no_notebook_sources_remain_in_the_repo() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let dir = std::path::Path::new(manifest).join("design/notebooks");
    assert!(
        !dir.exists() || std::fs::read_dir(&dir).unwrap().next().is_none(),
        "design/notebooks must stay empty here; move the book to the repo that owns it"
    );
}
