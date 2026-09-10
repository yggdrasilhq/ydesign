//! ydesign, the yggui base design language, shipped as live notebooks.
//!
//! Every Web/GUI project in the fleet builds on ONE design language:
//! Dioxus primitives, then the yggui component system (libyggterm), then an
//! app's own layer. ydesign is that base language made VISIBLE: a libyggterm
//! document-surface app whose notebooks exhibit the components, the canonical
//! patterns (sidebar partitioning, the row engine, forms), and the working
//! habit, consult your layer, fall through to the one below, grow the lower
//! layer when it lacks a component, and argue every visual judgement from a
//! pixel screenshot.

mod manifest;
mod notebook;
mod osc;
mod persist;
mod projects;
mod schema;
mod shelf_wiring_tests;
mod server;
mod trace;

use anyhow::Result;
use clap::Parser;
use serde_json::json;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use ytrace::Clock;

/// The declare cadence. yggterm expires a contribution after ~15s of silence,
/// so a killed app never leaves an overlay behind; ~4s is the contract's rate.
const HEARTBEAT: Duration = Duration::from_secs(4);

#[derive(Parser)]
#[command(
    name = "ydesign",
    version,
    about = "The yggui base design language as live notebooks (libyggterm document surface)"
)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
    /// Local registry of external project notebooks.
    #[arg(long, global = true)]
    config: Option<std::path::PathBuf>,
    /// Shelf mode: "guide" (the design language) or "examples" (canonical
    /// surfaces rebuilt as live schemas). Given explicitly, overrides the
    /// saved last-opened mode; otherwise the saved mode is restored.
    #[arg(long, value_parser = ["guide", "examples"])]
    mode: Option<String>,
    /// Print one shelf reading and exit, even inside yggterm.
    #[arg(long)]
    once: bool,
    /// With --once, print the raw JSON reading.
    #[arg(long)]
    json: bool,
    /// Print a notebook without a GUI. `--notebook` alone lists the shelf;
    /// `--notebook <id>` prints the page; add `--page <n>` when the notebook
    /// grows past one page.
    #[arg(long, num_args = 0..=1, default_missing_value = "")]
    notebook: Option<String>,
    /// Which page of `--notebook`, 1-based.
    #[arg(long)]
    page: Option<usize>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Create design notebooks without overwriting existing files, and register the repo.
    Init {
        repo: std::path::PathBuf,
        #[arg(long)]
        id: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = args.config.unwrap_or_else(projects::config_path);
    if let Some(Command::Init { repo, id }) = args.command {
        return projects::init(&repo, &id, &config);
    }
    let tracer = trace::init();
    let boot = Instant::now();

    let shelf_load = Instant::now();
    projects::load(&config)?;
    let notebooks = projects::notebooks().len();
    tracer.emit_span(
        "startup",
        "phase",
        "shelf_load",
        Clock::Wall,
        shelf_load.elapsed().as_millis() as f64,
        json!({"project_notebooks": notebooks}),
    );
    manifest::write_best_effort();

    if let Some(id) = args.notebook {
        return server::print_notebook(id.trim(), args.page);
    }

    let session = ["YGGTERM_SESSION_ID", "LC_YGGTERM_SESSION_ID"]
        .into_iter()
        .find_map(|key| std::env::var(key).ok().filter(|v| !v.is_empty()))
        .unwrap_or_default();
    if args.once || session.is_empty() {
        if session.is_empty() && !args.once {
            eprintln!(
                "ydesign: not running inside yggterm ($YGGTERM_SESSION_ID unset), \
                 printing the shelf instead of opening a surface."
            );
        }
        let saved_mode = persist::load().map(|saved| saved.mode);
        let mode = args
            .mode
            .or(saved_mode)
            .unwrap_or_else(|| schema::MODE_GUIDE.to_string());
        return server::print_once(&mode, "", args.json);
    }

    // The last opened state restores the reading place, mode, open page,
    // expanded groups, so a cold start reopens where the reader left off.
    // An explicit `--mode` still wins over the saved mode.
    let saved = persist::load();
    let mut view = saved
        .as_ref()
        .map(schema::View::restore)
        .unwrap_or_default();
    if let Some(mode) = &args.mode {
        view.select_mode(mode);
    }

    let spawn_started = Instant::now();
    let control = server::spawn(view)?;
    tracer.emit_span(
        "startup",
        "phase",
        "server_spawn",
        Clock::Wall,
        spawn_started.elapsed().as_millis() as f64,
        json!({}),
    );

    let running = Arc::new(AtomicBool::new(true));
    {
        let running = Arc::clone(&running);
        let session = session.clone();
        ctrlc::set_handler(move || {
            osc::emit_close(&session);
            running.store(false, Ordering::SeqCst);
        })?;
    }

    let mut first_declare = true;
    while running.load(Ordering::SeqCst) {
        let stamp = control.state.lock().unwrap().stamp;
        osc::emit_declare(&session, &control.url, &stamp.to_string());
        if first_declare {
            trace::phase(
                "first_declare",
                &boot,
                json!({"version": stamp.to_string()}),
            );
            first_declare = false;
        } else {
            trace::event(
                "declare",
                "heartbeat",
                json!({"version": stamp.to_string()}),
            );
        }
        std::thread::sleep(HEARTBEAT);
    }
    osc::emit_close(&session);
    trace::event("close", "emitted", json!({}));
    Ok(())
}
