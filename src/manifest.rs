//! ydesign's LAUNCHER MANIFEST — how the yggterm menus learn ydesign exists.
//!
//! Written to `~/.yggterm/apps/ydesign.json` on the app's OWN host on every
//! run, which repairs the binary path after an upgrade. The host's daemon scans
//! the directory and deletes manifests whose binary is gone — that is the whole
//! uninstall story. An app declares itself with a FILE, not by linking the
//! platform.

use anyhow::Result;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

fn manifest_value(binary: &Path) -> Value {
    json!({
        "name": "ydesign",
        "label": "Ydesign",
        "icon": "📐\u{fe0e}",
        "binary": binary.to_string_lossy(),
        // Exactly one verb is a row: the document surface the user opens and
        // returns to. `row_spawn` only controls the row context menu; the
        // titlebar `+` and start page still offer every verb.
        "verbs": [
            { "id": "new", "label": "New Ydesign", "args": [], "row_spawn": true },
        ],
    })
}

fn write_to(apps_dir: &Path, binary: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(apps_dir)?;
    let path = apps_dir.join("ydesign.json");
    std::fs::write(&path, serde_json::to_string_pretty(&manifest_value(binary))?)?;
    Ok(path)
}

/// Best-effort on every run; a failure must never stop the app.
pub fn write_best_effort() {
    let Some(home) = dirs::home_dir() else { return };
    let Ok(binary) = std::env::current_exe() else { return };
    let _ = write_to(&home.join(".yggterm").join("apps"), &binary);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_manifest_names_match_the_file_stem_and_the_binary_is_absolute() {
        let value = manifest_value(Path::new("/usr/local/bin/ydesign"));
        assert_eq!(value["name"], "ydesign");
        assert!(value["binary"].as_str().unwrap().starts_with('/'));
        assert_eq!(value["verbs"].as_array().unwrap().len(), 1);
    }

    /// The context menu's one launch affordance is deliberately a real row:
    /// ydesign is a foreground document-surface app and the row is how the
    /// user returns to it.
    #[test]
    fn the_one_verb_asks_to_become_a_sidebar_row() {
        let value = manifest_value(Path::new("/usr/local/bin/ydesign"));
        let verbs = value["verbs"].as_array().unwrap();
        assert_eq!(verbs[0]["label"], "New Ydesign");
        assert_eq!(verbs[0]["row_spawn"], serde_json::json!(true));
    }
}
