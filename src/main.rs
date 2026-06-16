use std::collections::HashMap;
use std::error::Error;

use lsp_server::{Connection, Message, Response};
use lsp_types::{
    CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, CompletionResponse,
    DidChangeTextDocumentParams, DidOpenTextDocumentParams, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind,
};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (conn, io) = Connection::stdio();

    let caps = serde_json::to_value(ServerCapabilities {
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(vec!["\\".into(), " ".into()]),
            ..Default::default()
        }),
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::FULL,
        )),
        ..Default::default()
    })?;

    conn.initialize(caps)?;

    let mut docs: HashMap<String, String> = HashMap::new();

    for msg in &conn.receiver {
        match msg {
            Message::Request(req) => {
                if conn.handle_shutdown(&req)? {
                    break;
                }
                if req.method == "textDocument/completion" {
                    let params: CompletionParams = serde_json::from_value(req.params.clone())?;
                    let uri = params.text_document_position.text_document.uri.to_string();
                    let items = docs
                        .get(&uri)
                        .map(|t| completions(t))
                        .unwrap_or_default();
                    let result = serde_json::to_value(CompletionResponse::Array(items))?;
                    conn.sender
                        .send(Message::Response(Response::new_ok(req.id, result)))?;
                }
            }
            Message::Notification(notif) => match notif.method.as_str() {
                "textDocument/didOpen" => {
                    let p: DidOpenTextDocumentParams = serde_json::from_value(notif.params)?;
                    docs.insert(p.text_document.uri.to_string(), p.text_document.text);
                }
                "textDocument/didChange" => {
                    let p: DidChangeTextDocumentParams = serde_json::from_value(notif.params)?;
                    if let Some(change) = p.content_changes.into_iter().last() {
                        docs.insert(p.text_document.uri.to_string(), change.text);
                    }
                }
                _ => {}
            },
            Message::Response(_) => {}
        }
    }

    io.join()?;
    Ok(())
}

// Parse every `let name = body` in the document and return two completions:
//   • the name itself  (so you can reference the binding)
//   • "name (expand)"  (inserts the full body in parentheses)
fn completions(text: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("--") {
            continue;
        }
        let Some(rest) = line.strip_prefix("let ") else {
            continue;
        };
        let Some(eq) = rest.find('=') else { continue };

        let name = rest[..eq].trim();
        let body = rest[eq + 1..].trim();

        if name.is_empty()
            || body.is_empty()
            || !name
                .chars()
                .all(|c| c.is_alphanumeric() || matches!(c, '_' | '#' | '\''))
        {
            continue;
        }

        items.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::VARIABLE),
            detail: Some(format!("= {}", body)),
            ..Default::default()
        });

        items.push(CompletionItem {
            label: format!("{} (expand)", name),
            kind: Some(CompletionItemKind::SNIPPET),
            insert_text: Some(format!("({})", body)),
            detail: Some(body.to_string()),
            ..Default::default()
        });
    }

    items
}
