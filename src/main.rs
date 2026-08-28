//! ydesign — the yggui base design language, shipped as live notebooks.
//!
//! Every Web/GUI project in the fleet builds on ONE design language:
//! Dioxus primitives, then the yggui component system (libyggterm), then an
//! app's own layer. ydesign is that base language made VISIBLE: a libyggterm
//! document-surface app whose notebooks exhibit the components, the canonical
//! patterns (sidebar partitioning, the row engine, forms), and the working
//! habit — consult your layer, fall through to the one below, grow the lower
//! layer when it lacks a component, and argue every visual judgement from a
//! pixel screenshot.

mod manifest;
mod notebook;
mod osc;
mod schema;
mod server;

use anyhow::Result;
use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

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
    /// Shelf mode: "guide" (the design language) or "examples" (canonical
    /// surfaces rebuilt as live schemas).
    #[arg(long, value_parser = ["guide", "examples"], default_value = "guide")]
    mode: String,
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

fn main() -> Result<()> {
    let args = Args::parse();
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
                "ydesign: not running inside yggterm ($YGGTERM_SESSION_ID unset) — \
                 printing the shelf instead of opening a surface."
            );
        }
        return server::print_once(&args.mode, "", args.json);
    }

    let control = server::spawn()?;
    {
        let mut pane = control.state.lock().unwrap();
        pane.view.select_mode(&args.mode);
    }

    let running = Arc::new(AtomicBool::new(true));
    {
        let running = Arc::clone(&running);
        let session = session.clone();
        ctrlc::set_handler(move || {
            osc::emit_close(&session);
            running.store(false, Ordering::SeqCst);
        })?;
    }

    while running.load(Ordering::SeqCst) {
        let stamp = control.state.lock().unwrap().stamp;
        osc::emit_declare(&session, &control.url, &stamp.to_string());
        std::thread::sleep(HEARTBEAT);
    }
    osc::emit_close(&session);
    Ok(())
}
