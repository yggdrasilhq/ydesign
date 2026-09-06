//! Registered design layers. Configuration is local; notebook sources stay in their repos.
//!
//! ⚡ LENIENT PER PROJECT, STRICT PER CONFIG. A structurally broken config
//! (bad JSON, duplicate or invalid id) is refused — the registry file must be
//! fixed. But one UNAVAILABLE PROJECT — a path that only exists on another
//! fleet host, a restructure in flight, a missing design/Inheritance.md — is
//! skipped with a warning while the rest of the shelf loads. Before
//! 2026-09-07 a single stale registry row hard-exited the whole app at cold
//! start; a registry row must never cost the app its surface.
use crate::notebook::{Notebook, Page};
use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::OnceLock,
};

static SHELF: OnceLock<Vec<Notebook>> = OnceLock::new();

#[derive(Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub projects: Vec<Project>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub id: String,
    pub path: PathBuf,
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ydesign/projects.json")
}

fn read_config(path: &Path) -> Result<Config> {
    if !path.exists() {
        return Ok(Config::default());
    }
    serde_json::from_slice(&std::fs::read(path)?).context("reading ydesign projects config")
}

pub fn load(path: &Path) -> Result<()> {
    let shelf = read_shelf(path)?;
    SHELF
        .set(shelf)
        .map_err(|_| anyhow::anyhow!("projects already loaded"))
}

fn read_shelf(path: &Path) -> Result<Vec<Notebook>> {
    let config = read_config(path)?;
    let mut ids = BTreeSet::new();
    let mut shelf = Vec::new();
    for project in config.projects {
        if !valid_id(&project.id) || !ids.insert(project.id.clone()) {
            bail!("invalid or duplicate project id: {}", project.id);
        }
        match load_project(&project, path.parent().unwrap_or(Path::new("."))) {
            Ok(mut notebooks) => shelf.append(&mut notebooks),
            Err(error) => eprintln!("ydesign: skipping project `{}`: {error:#}", project.id),
        }
    }
    Ok(shelf)
}

/// ONE project's notebooks; `Err` here skips the project, not the app.
fn load_project(project: &Project, config_dir: &Path) -> Result<Vec<Notebook>> {
    let root = if project.path.is_absolute() {
        project.path.clone()
    } else {
        config_dir.join(&project.path)
    }
    .canonicalize()
    .context("registered path is unavailable")?;
    let design = root
        .join("design")
        .canonicalize()
        .context("project needs design/")?;
    let inheritance = std::fs::read_to_string(design.join("Inheritance.md"))
        .context("project needs design/Inheritance.md")?;
    let mut files: Vec<_> = std::fs::read_dir(&design)?.collect::<std::io::Result<Vec<_>>>()?;
    files.sort_by_key(|entry| entry.file_name());
    let mut notebooks = Vec::new();
    for entry in files {
        let file = entry.path();
        if file.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        match load_notebook(&file, &design, &project.id, &inheritance) {
            Ok(notebook) => notebooks.push(notebook),
            Err(error) => eprintln!("ydesign: skipping `{}`: {error:#}", file.display()),
        }
    }
    Ok(notebooks)
}

/// ONE notebook file; `Err` here skips the file. An escaping file (an asset
/// or include reaching outside design/) is never served — it is skipped,
/// which keeps the containment guarantee while the rest of the shelf loads.
fn load_notebook(
    file: &Path,
    design: &Path,
    project_id: &str,
    inheritance: &str,
) -> Result<Notebook> {
    if !file.canonicalize()?.starts_with(design) {
        bail!("notebook escapes design directory");
    }
    let stem = file.file_stem().unwrap().to_string_lossy().to_string();
    let source = std::fs::read_to_string(file)?;
    let title = source
        .lines()
        .find_map(|l| l.strip_prefix("# "))
        .with_context(|| format!("{} needs a heading", file.display()))?
        .to_string();
    let markdown = resolve_assets(&source, design)?;
    let id = format!("project/{project_id}/{stem}");
    Ok(Notebook {
        id: id.clone(),
        title: format!("{project_id} · {title}"),
        mode: "guide".into(),
        description: inheritance.to_string(),
        author: project_id.to_string(),
        created_at_ms: 0,
        pages: vec![Page {
            id: format!("{id}-page"),
            title,
            markdown,
            composed: false,
        }],
    })
}

pub fn notebooks() -> Vec<Notebook> {
    SHELF.get().cloned().unwrap_or_default()
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn resolve_assets(source: &str, design: &Path) -> Result<String> {
    use base64::Engine;
    let mut out = source.to_string();
    // Explicit portable asset marker; never resolve arbitrary notebook URLs or fetch remote images.
    for tail in source.split("%ASSETS%/").skip(1) {
        let name = tail
            .split(|c: char| matches!(c, ')' | '"' | '`' | ']' | '(') || c.is_whitespace())
            .next()
            .unwrap_or("");
        let asset = design.join("assets").join(name).canonicalize()?;
        let asset_root = design.join("assets").canonicalize()?;
        if !asset.starts_with(asset_root) {
            bail!("asset escapes design/assets");
        }
        let mime = match asset.extension().and_then(|e| e.to_str()) {
            Some("png") => "image/png",
            Some("jpg" | "jpeg") => "image/jpeg",
            Some("webp") => "image/webp",
            Some("svg") => "image/svg+xml",
            _ => bail!("unsupported notebook image format"),
        };
        let bytes = std::fs::read(&asset)?;
        if bytes.len() > 8 * 1024 * 1024 {
            bail!("notebook asset exceeds 8 MiB");
        }
        out = out.replace(
            &format!("%ASSETS%/{name}"),
            &format!(
                "data:{mime};base64,{}",
                base64::engine::general_purpose::STANDARD.encode(bytes)
            ),
        );
    }
    Ok(out)
}

pub fn init(repo: &Path, id: &str, config: &Path) -> Result<()> {
    if !valid_id(id) {
        bail!("project id must use lowercase letters, digits and hyphens");
    }
    let root = repo
        .canonicalize()
        .context("repository must already exist")?;
    if !root.join(".git").exists() {
        bail!("target must be a git repository or worktree");
    }
    let mut registry = read_config(config)?;
    if let Some(existing) = registry.projects.iter().find(|p| p.id == id) {
        let old = if existing.path.is_absolute() {
            existing.path.clone()
        } else {
            config
                .parent()
                .unwrap_or(Path::new("."))
                .join(&existing.path)
        };
        if old.canonicalize()? != root {
            bail!("project id already registered at another path");
        }
    }
    let design = root.join("design");
    std::fs::create_dir_all(design.join("assets"))?;
    let files = [
        (root.join("DESIGN.md"), "# Design guide\n\nRead design/Inheritance.md, then the notebooks in design/. Visual identity, palette, typography and examples live in those notebooks. Consult the inherited ydesign base notebooks for undefined decisions. Preview with ydesign --notebook; register this repository with ydesign init.\n".to_string()),
        (design.join("Inheritance.md"), format!("# Inheritance\n\nLayer: {id}\n\nParent: yggui (ydesign base notebooks). Transitive chain: Dioxus components → yggui → {id}. Read the parent Inheritance.md before overriding a decision.\n\nLocal notebooks override only decisions they explicitly name. All other rules inherit. Multiple parents must name their order and resolve overlaps explicitly; cycles and unresolved conflicts are invalid. This file records provenance and override scope, not palette or typography.\n")),
        (design.join("00-brand.md"), format!("# {id} — brand identity\n\nStatus: inherited baseline; app-specific choices are not yet approved.\n\nInherit yggui semantic colors, system typography, focus and interaction behavior. Record app-specific palette, typography, imagery and rationale here when chosen. Add a large rendered specimen and keyboard/mouse walkthrough for each material override.\n")),
        (design.join("assets/.gitkeep"), String::new()),
    ];
    for (path, body) in files {
        use std::io::Write;
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut f) => {
                f.write_all(body.as_bytes())?;
                println!("created {}", path.display());
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                println!("preserved {}", path.display())
            }
            Err(e) => return Err(e.into()),
        }
    }
    if !registry.projects.iter().any(|p| p.id == id) {
        registry.projects.push(Project {
            id: id.into(),
            path: root,
        });
        if let Some(parent) = config.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(config, serde_json::to_vec_pretty(&registry)?)?;
    }
    println!("registered {id} in {}", config.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn project_namespace_cannot_shadow_base_or_escape() {
        for bad in ["", "../gallery", "a/b", "A", "a:b"] {
            assert!(!valid_id(bad));
        }
        assert!(valid_id("sample-app"));
        assert!(serde_json::from_str::<Config>(r#"{"projects":[],"typo":true}"#).is_err());
    }

    #[test]
    fn init_preserves_work_and_external_notebooks_resolve_assets() {
        let scratch = dirs::home_dir().unwrap().join(".yggterm/scratchpad");
        let root = scratch.join(format!(
            "ydesign-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(root.join("repo/.git")).unwrap();
        let repo = root.join("repo");
        let config = root.join("registry.json");
        std::fs::write(repo.join("DESIGN.md"), "# Existing route\n").unwrap();
        init(&repo, "sample", &config).unwrap();
        init(&repo, "sample", &config).unwrap();
        assert_eq!(
            std::fs::read_to_string(repo.join("DESIGN.md")).unwrap(),
            "# Existing route\n"
        );
        assert_eq!(read_config(&config).unwrap().projects.len(), 1);
        std::fs::write(
            repo.join("design/assets/sample.svg"),
            "<svg xmlns=\"http://www.w3.org/2000/svg\"/>",
        )
        .unwrap();
        std::fs::write(
            repo.join("design/gallery.md"),
            "# External gallery\n\n![Example](%ASSETS%/sample.svg)",
        )
        .unwrap();
        let shelf = read_shelf(&config).unwrap();
        let page = &shelf
            .iter()
            .find(|n| n.id == "project/sample/gallery")
            .unwrap()
            .pages[0];
        assert!(page.markdown.contains("data:image/svg+xml;base64,"));
        assert!(
            resolve_assets(
                "[`%ASSETS%/sample.svg`](%ASSETS%/sample.svg)",
                &repo.join("design")
            )
            .unwrap()
            .contains("data:image/svg+xml;base64,")
        );
        assert!(shelf.iter().all(|n| n.id.starts_with("project/sample/")));
        std::fs::remove_dir_all(root).unwrap();
    }

    /// A stale registry row (a path that exists only on another fleet host)
    /// is SKIPPED, never fatal — and an escaping notebook file is dropped
    /// while its siblings still load. The app must open either way.
    #[test]
    fn one_unavailable_project_or_escaping_file_never_kills_the_shelf() {
        let scratch = dirs::home_dir().unwrap().join(".yggterm/scratchpad");
        let root = scratch.join(format!(
            "ydesign-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(root.join("repo/design")).unwrap();
        let repo = root.join("repo");
        let config = root.join("registry.json");
        std::fs::write(repo.join("design/Inheritance.md"), "# Inheritance\n").unwrap();
        std::fs::write(repo.join("design/gallery.md"), "# External gallery\n").unwrap();
        let registry = Config {
            projects: vec![
                Project {
                    id: "sample".into(),
                    path: repo.clone(),
                },
                Project {
                    id: "ghost".into(),
                    path: root.join("does-not-exist"),
                },
            ],
        };
        std::fs::write(&config, serde_json::to_vec_pretty(&registry).unwrap()).unwrap();
        // Force the escape: a canonicalize check reads through symlinks, so
        // the notebook IS a link to a file outside design/.
        #[cfg(unix)]
        std::os::unix::fs::symlink(root.join("registry.json"), repo.join("design/escape.md"))
            .unwrap();

        let shelf = read_shelf(&config).unwrap();
        assert!(
            shelf.iter().all(|n| n.id.starts_with("project/sample/")),
            "the ghost project is skipped with a warning"
        );
        assert!(
            shelf.iter().any(|n| n.id == "project/sample/gallery"),
            "the escaping file is skipped, its sibling loads"
        );
        assert!(
            !shelf.iter().any(|n| n.id.ends_with("/escape")),
            "an escaping notebook is never served"
        );

        // A whole project mid-restructure (no Inheritance.md) skips cleanly too.
        std::fs::remove_file(repo.join("design/Inheritance.md")).unwrap();
        assert!(read_shelf(&config).unwrap().is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }
}
