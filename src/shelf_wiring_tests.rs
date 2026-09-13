//! Wiring tests: the shelf may not silently drop a book.
//!
//! 2026-09-07: the Icons and ZCode-reversed notebooks sat in `design/notebooks/`
//! for a day without an entry in `base_notebooks()`, written, reviewed, and
//! unreachable from the app. This test makes that class fail the build: every
//! notebook file under `design/notebooks/` must be embedded by `src/notebook.rs`
//! (each shows up exactly once as `include_str!("../design/notebooks/<name>.emd")`).

#[test]
fn every_notebook_file_is_wired_into_the_shelf() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let dir = std::path::Path::new(manifest).join("design/notebooks");
    let notebook_rs = std::fs::read_to_string(std::path::Path::new(manifest).join("src/notebook.rs"))
        .expect("src/notebook.rs is readable");
    let mut missing = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .expect("design/notebooks exists")
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    entries.sort();
    for name in &entries {
        if !name.ends_with(".emd") {
            continue;
        }
        let needle = format!("../design/notebooks/{name}");
        if !notebook_rs.contains(&needle) {
            missing.push(name.clone());
        }
    }
    assert!(
        missing.is_empty(),
        "notebooks present on disk but never wired into base_notebooks(), \
         add an entry or a documented allowlist row: {missing:?}"
    );
}
