//! ydesign's control server and action handler.
//!
//! `GET /ping` (liveness + change stamp), `GET /pane/<id>` (the widget schema),
//! `POST /action` (mode switch, page turns, specimen controls).
//!
//! ⚡ THE ACTION REPLY REPAINTS. A mutating action replies with the POSTing
//! pane's schema plus `refetch_document: true` — the GUI applies both at once
//! (`AppPaneActionReply`). The GUI only re-fetches on the `document_version`
//! EDGE, which it observes on the next heartbeat declare (≤4s) or ping
//! (~2.5s); before this reply shape every shelf click sat through that wait,
//! which read as a laggingly-populating sidebar. Measured and fixed
//! 2026-09-07 with the ytrace probes in `trace.rs`.

use crate::{notebook, persist, schema, trace};
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// libyggterm carries a clicked widget's value alongside draft inputs. Accept
/// the former top-level shape as well so older clients and direct integrations
/// remain compatible during fleet rollouts.
fn action_value(body: &Value) -> &str {
    body.get("values")
        .and_then(|values| values.get("value"))
        .and_then(Value::as_str)
        .or_else(|| body.get("value").and_then(Value::as_str))
        .unwrap_or("")
}

pub struct PaneState {
    pub view: schema::View,
    pub stamp: u64,
    /// Where the last-opened state persists. Every accepted action saves; the
    /// file is tiny and the next cold start reopens exactly this reading place.
    pub state_path: PathBuf,
}

impl PaneState {
    fn touch(&mut self) {
        self.stamp = self.stamp.wrapping_add(1);
        persist::save_to(&self.state_path, &self.view);
    }

    fn schema_for(&self, pane_id: &str) -> Value {
        if pane_id == "rail" {
            schema::rail_view(&self.view)
        } else {
            schema::viewport_view(&self.view)
        }
    }

    /// The wire reply for one handled action. A mutation repaints the POSTing
    /// pane immediately (the returned `schema`) and refetches the document
    /// surface immediately (`refetch_document`) — neither waits for the next
    /// heartbeat. The stamp is only a change DETECTOR; the schema in the reply
    /// is the app's current truth whatever stamp it is filed under.
    fn action_reply(&self, pane_id: &str, mutated: bool) -> Value {
        if !mutated {
            return json!({ "ok": true });
        }
        json!({
            "ok": true,
            "schema": self.schema_for(pane_id),
            // Redundant when the action came from the document itself (its
            // schema is already in the reply) and load-bearing when it came
            // from the rail — the viewport must turn the page NOW.
            "refetch_document": pane_id != "design",
        })
    }
}

pub struct Server {
    pub url: String,
    pub state: Arc<Mutex<PaneState>>,
}

pub fn spawn(view: schema::View) -> Result<Server> {
    let listener =
        TcpListener::bind("127.0.0.1:0").context("binding the ydesign control server")?;
    let port = listener.local_addr()?.port();
    let state = Arc::new(Mutex::new(PaneState {
        view,
        stamp: 0,
        state_path: persist::state_path(),
    }));
    {
        let state = Arc::clone(&state);
        std::thread::spawn(move || {
            for incoming in listener.incoming() {
                let Ok(stream) = incoming else { continue };
                let state = Arc::clone(&state);
                std::thread::spawn(move || handle_conn(stream, state));
            }
        });
    }
    Ok(Server {
        url: format!("http://127.0.0.1:{port}"),
        state,
    })
}

fn handle_conn(stream: TcpStream, state: Arc<Mutex<PaneState>>) {
    let started = Instant::now();
    let Ok(peek) = stream.try_clone() else { return };
    let mut reader = BufReader::new(peek);
    let mut line = String::new();
    if reader.read_line(&mut line).is_err() {
        return;
    }
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let target = parts.next().unwrap_or("/");
    let (path, _query) = target.split_once('?').unwrap_or((target, ""));
    let path = path.to_string();

    let mut content_length = 0usize;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).is_err() || header.trim().is_empty() {
            break;
        }
        if let Some((name, value)) = header.split_once(':')
            && name.eq_ignore_ascii_case("content-length")
        {
            content_length = value.trim().parse().unwrap_or(0);
        }
    }
    let body: Value = if content_length > 0 {
        let mut raw = vec![0u8; content_length];
        if reader.read_exact(&mut raw).is_err() {
            return;
        }
        serde_json::from_slice(&raw).unwrap_or(Value::Null)
    } else {
        Value::Null
    };

    match (method.as_str(), path.as_str()) {
        ("GET", "/ping") => {
            let pane = state.lock().unwrap();
            let bytes = respond(
                stream,
                200,
                &json!({
                    "ok": true,
                    "app_name": "Ydesign",
                    "document_version": pane.stamp.to_string(),
                }),
            );
            trace::span_ms(
                "pane_fetch",
                "ping",
                started.elapsed().as_millis(),
                json!({"bytes": bytes}),
            );
        }
        ("GET", "/pane/design") | ("GET", "/pane/rail") => {
            let pane_id = path.rsplit('/').next().unwrap_or("design").to_string();
            let body = {
                let pane = state.lock().unwrap();
                pane.schema_for(&pane_id)
            };
            let bytes = respond(stream, 200, &body);
            trace::span_ms(
                "pane_fetch",
                &pane_id,
                started.elapsed().as_millis(),
                json!({"bytes": bytes, "widgets": body["widgets"].as_array().map(|w| w.len())}),
            );
        }
        ("POST", "/action") => {
            let action = body["action"].as_str().unwrap_or("").to_string();
            let value = action_value(&body).to_string();
            let pane_id = body["pane"].as_str().unwrap_or("design").to_string();
            let mut pane = state.lock().unwrap();
            let before = pane.stamp;

            if pane.view.book_action(&action) || pane.view.study_action(&action) {
                pane.touch();
                let reply = pane.action_reply(&pane_id, true);
                let bytes = respond(stream, 200, &reply);
                trace::span_ms(
                    "action",
                    &action,
                    started.elapsed().as_millis(),
                    json!({
                        "pane": pane_id, "mutated": true, "bytes": bytes,
                    }),
                );
                return;
            }

            match action.as_str() {
                "mode" => {
                    if pane.view.select_mode(&value) {
                        pane.touch();
                    } else {
                        pane.view.notice = Some("Unknown ydesign mode".to_string());
                    }
                }
                "refresh" => {
                    pane.touch();
                }
                other => {
                    // Specimen controls and shelf rows. The verb prefix names
                    // the family; every one of them only proves the round
                    // trip — an action POSTed, a reply schema repainted.
                    let named = other
                        .split_once(':')
                        .map(|(family, _)| family)
                        .unwrap_or(other);
                    match named {
                        "page_open" => {
                            if let Some(rest) = other.strip_prefix("page_open:")
                                && let Some((nb_id, idx_str)) = rest.split_once(':')
                                && let Ok(idx) = idx_str.parse::<usize>()
                            {
                                if let Some(nb) = notebook::get_notebook(nb_id)
                                    && let Some(page) = nb.pages.get(idx)
                                {
                                    pane.view.selected_book = None;
                                    pane.view.selected_notebook = Some(nb_id.to_string());
                                    pane.view.selected_page = Some(page.id.clone());
                                    pane.view.notice =
                                        Some(format!("📖 {} — {}", nb.title, page.title));
                                }
                                pane.touch();
                            }
                        }
                        "demo" | "demo_tab" | "notebook" | "specimen_row" | "live_row"
                        | "file_row" => {
                            pane.view.notice = Some(format!(
                                "Specimen control “{value}” answered — an action POSTed, a schema replied."
                            ));
                            pane.touch();
                        }
                        _ => {}
                    }
                }
            }

            let mutated = pane.stamp != before;
            let reply = pane.action_reply(&pane_id, mutated);
            let bytes = respond(stream, 200, &reply);
            trace::span_ms(
                "action",
                &action,
                started.elapsed().as_millis(),
                json!({
                    "pane": pane_id, "mutated": mutated, "bytes": bytes,
                }),
            );
        }
        _ => {
            respond(stream, 404, &json!({"error": "not found"}));
        }
    }
}

fn respond(mut stream: TcpStream, status: u16, body: &Value) -> usize {
    let payload = body.to_string();
    let head = format!(
        "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n",
        payload.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(payload.as_bytes());
    let _ = stream.flush();
    payload.len()
}

/// Print a notebook, or one of its pages, without a GUI.
///
/// ⭐ THE NOTEBOOKS ARE READINGS, SO THEY ARE CHECKABLE LIKE ONE. A page that
/// can only be seen inside a running window cannot be verified without
/// interrupting whoever is using that window. `ydesign --notebook` lists the
/// shelf; `--notebook <id>` prints the page; CI reads both.
pub fn print_notebook(id: &str, page: Option<usize>) -> Result<()> {
    if id.is_empty() {
        for nb in notebook::list_notebooks(None) {
            println!(
                "{:<22} [{:<8}] {:<2} page(s)  {}",
                nb.id,
                nb.mode,
                nb.pages.len(),
                nb.title
            );
        }
        return Ok(());
    }
    let Some(nb) = notebook::get_notebook(id) else {
        anyhow::bail!("no notebook `{id}` — run `ydesign --notebook` to list the shelf");
    };
    let Some(n) = page else {
        println!("📖 {}  [{}]\n{}\n", nb.title, nb.mode, nb.description);
        for (idx, p) in nb.pages.iter().enumerate() {
            println!("  {}. {}", idx + 1, p.title);
        }
        println!("\nOne page: ydesign --notebook {} --page 1", nb.id);
        return Ok(());
    };
    let Some(p) = n.checked_sub(1).and_then(|i| nb.pages.get(i)) else {
        anyhow::bail!(
            "notebook `{}` has {} page(s); asked for {n}",
            nb.id,
            nb.pages.len()
        );
    };
    println!("{}", p.markdown);
    Ok(())
}

/// The standalone degradation: no `$YGGTERM_SESSION_ID`, no surface — print
/// the shelf and say how to open it for real. Never a half-open window.
pub fn print_once(_mode: &str, _tab: &str, as_json: bool) -> Result<()> {
    let shelf = notebook::list_notebooks(None);
    if as_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "app": "ydesign",
                "surface": "none (standalone)",
                "notebooks": shelf.iter().map(|nb| json!({
                    "id": nb.id,
                    "mode": nb.mode,
                    "title": nb.title,
                    "pages": nb.pages.len(),
                })).collect::<Vec<_>>(),
            }))?
        );
        return Ok(());
    }
    println!("ydesign — the yggui base design language");
    println!("Not running inside yggterm ($YGGTERM_SESSION_ID unset); printing the shelf.");
    println!();
    for nb in &shelf {
        println!("  {:<22} [{:<8}] {}", nb.id, nb.mode, nb.title);
    }
    println!();
    println!("Inside yggterm: run `ydesign` to open the notebooks as a surface.");
    println!("Read one here:  ydesign --notebook <id> --page 1");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{PaneState, action_value};
    use crate::{persist, schema};
    use serde_json::json;

    #[test]
    fn action_value_reads_the_libyggterm_values_envelope() {
        let body = json!({"action": "mode", "values": {"value": "examples"}});
        assert_eq!(action_value(&body), "examples");
    }

    #[test]
    fn action_value_keeps_legacy_top_level_compatibility() {
        let body = json!({"action": "mode", "value": "guide"});
        assert_eq!(action_value(&body), "guide");
    }

    fn test_pane() -> PaneState {
        let scratch = dirs::home_dir()
            .unwrap()
            .join(".yggterm/scratchpad")
            .join(format!(
                "ydesign-server-test-{}-{}.json",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
        PaneState {
            view: schema::View::default(),
            stamp: 0,
            state_path: scratch,
        }
    }

    #[test]
    fn a_mutating_rail_action_replies_with_schema_and_immediate_refetch() {
        let mut pane = test_pane();
        assert!(pane.view.book_action("book_toggle:yggui"));
        pane.touch();
        let reply = pane.action_reply("rail", true);
        assert_eq!(reply["ok"], true);
        // The rail repaints from the reply itself — no heartbeat wait.
        let widgets = reply["schema"]["widgets"].as_array().expect("rail schema");
        assert!(widgets.iter().any(|w| w["id"] == "book:yggui"));
        // The viewport refetches NOW instead of on the next version edge.
        assert_eq!(reply["refetch_document"], true);
    }

    #[test]
    fn a_noop_action_is_a_bare_ok_and_a_document_action_skips_the_refetch() {
        let pane = test_pane();
        let noop = pane.action_reply("rail", false);
        assert_eq!(noop["ok"], true);
        assert!(noop.get("schema").is_none());
        assert!(noop.get("refetch_document").is_none());
        let from_document = pane.action_reply("design", true);
        assert!(from_document["schema"].is_object());
        // The reply schema already repaints the posting document pane.
        assert_eq!(from_document["refetch_document"], false);
    }

    #[test]
    fn touch_persists_the_reading_place() {
        let mut pane = test_pane();
        assert!(pane.view.book_action("book_open:yggui"));
        pane.touch();
        let saved = persist::load_from(&pane.state_path).expect("touch must save");
        assert_eq!(saved.selected_book.as_deref(), Some("yggui"));
        let restored = schema::View::restore(&saved);
        assert_eq!(restored.selected_book.as_deref(), Some("yggui"));
        std::fs::remove_file(&pane.state_path).ok();
    }
}
