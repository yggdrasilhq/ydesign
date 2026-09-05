//! ydesign's control server and action handler.
//!
//! `GET /ping` (liveness + change stamp), `GET /pane/<id>` (the widget schema),
//! `POST /action` (mode switch, page turns, specimen controls).

use crate::{notebook, schema};
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

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
}

impl PaneState {
    fn touch(&mut self) {
        self.stamp = self.stamp.wrapping_add(1);
    }
}

pub struct Server {
    pub url: String,
    pub state: Arc<Mutex<PaneState>>,
}

pub fn spawn() -> Result<Server> {
    let listener = TcpListener::bind("127.0.0.1:0").context("binding the ydesign control server")?;
    let port = listener.local_addr()?.port();
    let state = Arc::new(Mutex::new(PaneState {
        view: schema::View::default(),
        stamp: 0,
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
    let Ok(peek) = stream.try_clone() else { return };
    let mut reader = BufReader::new(peek);
    let mut line = String::new();
    if reader.read_line(&mut line).is_err() {
        return;
    }
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");
    let (path, _query) = target.split_once('?').unwrap_or((target, ""));

    let mut content_length = 0usize;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).is_err() || header.trim().is_empty() {
            break;
        }
        if let Some((name, value)) = header.split_once(':')
            && name.eq_ignore_ascii_case("content-length") {
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

    match (method, path) {
        ("GET", "/ping") => {
            let pane = state.lock().unwrap();
            respond(
                stream,
                200,
                &json!({
                    "ok": true,
                    "app_name": "Ydesign",
                    "document_version": pane.stamp.to_string(),
                }),
            );
        }
        ("GET", "/pane/design") => {
            let view = state.lock().unwrap().view.clone();
            respond(stream, 200, &schema::viewport_view(&view));
        }
        ("GET", "/pane/rail") => {
            let view = state.lock().unwrap().view.clone();
            respond(stream, 200, &schema::rail_view(&view));
        }
        ("POST", "/action") => {
            let action = body["action"].as_str().unwrap_or("");
            let value = action_value(&body);
            let mut pane = state.lock().unwrap();

            if pane.view.study_action(action) {
                pane.touch();
                respond(stream, 200, &json!({"ok":true, "schema":schema::viewport_view(&pane.view)}));
                return;
            }

            match action {
                "mode" => {
                    if pane.view.select_mode(value) {
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
                                    && let Ok(idx) = idx_str.parse::<usize>() {
                                        pane.view.selected_notebook = Some(nb_id.to_string());
                                        if let Some(nb) = notebook::get_notebook(nb_id)
                                            && let Some(page) = nb.pages.get(idx) {
                                                pane.view.selected_page = Some(page.id.clone());
                                                pane.view.notice = Some(format!(
                                                    "📖 {} — {}",
                                                    nb.title, page.title
                                                ));
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

            respond(stream, 200, &json!({"ok": true}));
        }
        _ => {
            respond(stream, 404, &json!({"error": "not found"}));
        }
    }
}

fn respond(mut stream: TcpStream, status: u16, body: &Value) {
    let payload = body.to_string();
    let head = format!(
        "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n",
        payload.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(payload.as_bytes());
    let _ = stream.flush();
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
    println!(
        "Not running inside yggterm ($YGGTERM_SESSION_ID unset); printing the shelf."
    );
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
    use super::action_value;
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
}
