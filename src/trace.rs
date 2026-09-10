//! ytrace probes, ydesign's side of every surface timeline.
//!
//! The GUI's traces show a declare arriving and a pane being fetched; these
//! records carry the APP side, cold-start phases, schema build cost, action
//! handling, the declare cadence, into the same query plane, so a populate
//! or repaint complaint can be answered from one trace read instead of a
//! code argument. JSONL lands at `~/.local/share/ytrace/ydesign/ytrace.jsonl`
//! on the host the app runs on (`ytrace query --name … --json` to read it).
//!
//! The 2026-09-07 populate-lag diagnosis was made from the contract alone
//! because the app side was invisible; these probes exist so the next lag is
//! measured, not argued. Payloads carry counts and durations, never paths.
use std::sync::OnceLock;
use ytrace::{Clock, Provider, Sample};

static TRACER: OnceLock<Provider> = OnceLock::new();

/// Every probe the app can emit. Spans are gate-checked against this table
/// (`emit_span` samples through it); events append regardless, but a row here
/// keeps one honest inventory.
const PROBES: &[&str] = &[
    "startup/phase",
    "surface/declare",
    "surface/pane_fetch",
    "surface/action",
    "surface/close",
];

pub fn init() -> &'static Provider {
    TRACER.get_or_init(|| {
        let provider = Provider::new("ydesign", env!("CARGO_PKG_VERSION"));
        for probe in PROBES {
            provider.register(probe, Clock::Wall, Sample::always());
        }
        provider
    })
}

pub fn phase(name: &str, started: &std::time::Instant, payload: serde_json::Value) {
    init().emit_span(
        "startup",
        "phase",
        name,
        Clock::Wall,
        started.elapsed().as_millis() as f64,
        payload,
    );
}

pub fn event(category: &str, name: &str, payload: serde_json::Value) {
    init().event("surface", category, name, payload);
}

pub fn span_ms(category: &str, name: &str, ms: u128, payload: serde_json::Value) {
    init().emit_span("surface", category, name, Clock::Wall, ms as f64, payload);
}

#[cfg(test)]
mod tests {
    #[test]
    fn probes_are_well_formed_category_slash_name() {
        for probe in super::PROBES {
            let (category, name) = probe
                .split_once('/')
                .unwrap_or_else(|| panic!("probe `{probe}` must be category/name"));
            assert!(!category.is_empty() && !name.is_empty());
        }
    }

    #[test]
    fn the_tracer_initializes_and_writes_a_record() {
        // The provider creates its home lazily on first emission; one event
        // must land without panicking even before any declare fires.
        let tracer = super::init();
        tracer.event(
            "surface",
            "test",
            "heartbeat",
            serde_json::json!({"probe": true}),
        );
    }
}
