//! The last opened state — the reading place a cold start restores.
//!
//! `~/.config/ydesign/state.json`, next to the `projects.json` registry,
//! written best-effort on every accepted action (the file is a few hundred
//! bytes) and loaded once at boot. Owner direction 2026-09-07: ydesign must
//! reopen where the reader left off, not reset to the home page every launch.
//! Saved ids are re-validated against the current shelf on load — a notebook
//! that was renamed or removed falls back to the home page, never a stale id.
use crate::schema::View;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SavedView {
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub selected_book: Option<String>,
    #[serde(default)]
    pub selected_notebook: Option<String>,
    #[serde(default)]
    pub selected_page: Option<String>,
    #[serde(default)]
    pub expanded_books: Vec<String>,
}

pub fn state_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ydesign/state.json")
}

pub fn load() -> Option<SavedView> {
    load_from(&state_path())
}

pub fn load_from(path: &Path) -> Option<SavedView> {
    let raw = std::fs::read(path).ok()?;
    serde_json::from_slice(&raw).ok()
}

pub fn save_to(path: &Path, view: &View) {
    let saved = SavedView {
        mode: view.mode.clone(),
        selected_book: view.selected_book.clone(),
        selected_notebook: view.selected_notebook.clone(),
        selected_page: view.selected_page.clone(),
        expanded_books: view.expanded_books.iter().cloned().collect(),
    };
    if let Ok(bytes) = serde_json::to_vec_pretty(&saved) {
        write_atomic(path, &bytes);
    }
}

/// Temp-then-rename so a killed app can never leave a half-written state file
/// (the same discipline the fleet deploy convention uses for binaries).
fn write_atomic(path: &Path, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let tmp = path.with_extension("json.new");
    if std::fs::write(&tmp, bytes).is_ok() {
        let _ = std::fs::rename(&tmp, path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch_path(tag: &str) -> PathBuf {
        let scratch = dirs::home_dir().unwrap().join(".yggterm/scratchpad");
        std::fs::create_dir_all(&scratch).unwrap();
        scratch.join(format!(
            "ydesign-state-test-{}-{}-{}.json",
            tag,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn a_saved_view_survives_a_round_trip() {
        let path = scratch_path("roundtrip");
        let view = View::default();
        save_to(&path, &view);
        let loaded = load_from(&path).expect("saved state must load");
        let restored = View::restore(&loaded);
        assert_eq!(restored.mode, view.mode);
        assert_eq!(restored.selected_notebook, view.selected_notebook);
        assert_eq!(restored.expanded_books, view.expanded_books);
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn a_missing_or_corrupt_state_file_reads_as_none() {
        assert!(load_from(&scratch_path("absent")).is_none());
        let path = scratch_path("corrupt");
        std::fs::write(&path, b"{ not json").unwrap();
        assert!(load_from(&path).is_none());
        std::fs::remove_file(&path).unwrap();
    }
}
