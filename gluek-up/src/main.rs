use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

mod ui;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Selection {
    pub line: u32,
    pub column: u32,
    pub anchor_line: u32,
    pub anchor_column: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Attention {
    pub file: String,
    pub selections: Vec<Selection>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContextResponse {
    pub app_id: Option<String>,
    pub pid: Option<u32>,
    pub attention: Option<Attention>,
}

#[derive(Debug, Clone)]
pub struct MatchItem {
    pub file_path: String,
    pub line: u32,
    pub column: u32,
    pub label: String,      // e.g. "struct Selection"
    pub snippet: String,    // line preview
}

fn percent_decode(s: &str) -> String {
    let mut bytes = Vec::new();
    let mut bytes_iter = s.as_bytes().iter();
    while let Some(&b) = bytes_iter.next() {
        if b == b'%' {
            let h1 = bytes_iter.next().copied();
            let h2 = bytes_iter.next().copied();
            if let (Some(h1), Some(h2)) = (h1, h2) {
                if let Ok(val) = u8::from_str_radix(std::str::from_utf8(&[h1, h2]).unwrap_or("00"), 16) {
                    bytes.push(val);
                    continue;
                }
            }
            bytes.push(b'%');
            if let Some(x) = h1 { bytes.push(x); }
            if let Some(x) = h2 { bytes.push(x); }
        } else {
            bytes.push(b);
        }
    }
    String::from_utf8(bytes).unwrap_or_else(|_| s.to_string())
}

fn fetch_context() -> Result<ContextResponse, Box<dyn std::error::Error>> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| "/run/user/1000".to_string());
    let socket_path = Path::new(&runtime_dir).join("current.sock");
    
    let mut stream = UnixStream::connect(socket_path)?;
    let query_req = json!({ "type": "Query" });
    stream.write_all(format!("{}\n", query_req).as_bytes())?;
    
    let mut response_buf = String::new();
    stream.read_to_string(&mut response_buf)?;
    
    let context: ContextResponse = serde_json::from_str(&response_buf)?;
    Ok(context)
}

fn parse_location(val: &Value) -> Result<(String, u32, u32), Box<dyn std::error::Error>> {
    let uri = val.get("uri")
        .or_else(|| val.get("targetUri"))
        .and_then(|u| u.as_str())
        .ok_or_else(|| "Missing location URI")?
        .to_string();
        
    let range = val.get("range")
        .or_else(|| val.get("targetSelectionRange"))
        .or_else(|| val.get("targetRange"))
        .ok_or_else(|| "Missing range")?;
        
    let start = range.get("start")
        .ok_or_else(|| "Missing start position")?;
        
    let line = start.get("line")
        .and_then(|l| l.as_u64())
        .ok_or_else(|| "Missing line")? as u32 + 1; // Convert to 1-indexed
        
    let character = start.get("character")
        .and_then(|c| c.as_u64())
        .ok_or_else(|| "Missing character")? as u32 + 1; // Convert to 1-indexed
        
    Ok((uri, line, character))
}

fn get_file_line(file_path: &str, line_num: u32) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for (idx, line) in reader.lines().enumerate() {
        if idx as u32 == line_num - 1 {
            return Ok(line?);
        }
    }
    Err("Line not found".into())
}

fn fetch_broker_matches(
    is_defs: bool,
    active_file: &str,
    line: u32,
    col: u32,
) -> Result<Vec<MatchItem>, Box<dyn std::error::Error>> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| "/run/user/1000".to_string());
    let socket_path = Path::new(&runtime_dir).join("lsp-broker-query.sock");
    
    let mut stream = UnixStream::connect(socket_path)?;
    
    let method = if is_defs { "textDocument/definition" } else { "textDocument/references" };
    let params = if is_defs {
        json!({
            "textDocument": { "uri": format!("file://{}", active_file) },
            "position": { "line": line.saturating_sub(1), "character": col.saturating_sub(1) }
        })
    } else {
        json!({
            "textDocument": { "uri": format!("file://{}", active_file) },
            "position": { "line": line.saturating_sub(1), "character": col.saturating_sub(1) },
            "context": { "includeDeclaration": true }
        })
    };
    
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": params
    });
    
    let request_body = serde_json::to_string(&request)?;
    let payload = format!("Content-Length: {}\r\n\r\n{}", request_body.len(), request_body);
    stream.write_all(payload.as_bytes())?;
    stream.flush()?;
    
    let mut reader = BufReader::new(stream);
    let mut content_length = None;
    let mut header_line = String::new();
    
    loop {
        header_line.clear();
        let bytes_read = reader.read_line(&mut header_line)?;
        if bytes_read == 0 {
            return Err("EOF reading header".into());
        }
        if header_line == "\r\n" || header_line == "\n" {
            break;
        }
        if header_line.to_lowercase().starts_with("content-length:") {
            let parts: Vec<&str> = header_line.split(':').collect();
            if parts.len() >= 2 {
                content_length = Some(parts[1].trim().parse::<usize>()?);
            }
        }
    }
    
    let length = content_length.ok_or_else(|| "Missing Content-Length header")?;
    let mut body = vec![0u8; length];
    reader.read_exact(&mut body)?;
    
    let response_str = String::from_utf8(body)?;
    let response: Value = serde_json::from_str(&response_str)?;
    
    if let Some(err) = response.get("error") {
        return Err(format!("LSP error: {:?}", err).into());
    }
    
    let result = response.get("result").ok_or_else(|| "Missing result field")?;
    
    let mut locations = Vec::new();
    if result.is_object() {
        locations.push(parse_location(result)?);
    } else if let Some(arr) = result.as_array() {
        for val in arr {
            if let Ok(loc) = parse_location(val) {
                locations.push(loc);
            }
        }
    }
    
    let mut matches = Vec::new();
    for (uri, start_line, start_col) in locations {
        let raw_path = uri.trim_start_matches("file://");
        let file_path = percent_decode(raw_path);
        let snippet = get_file_line(&file_path, start_line).unwrap_or_default();
        let file_name = Path::new(&file_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&file_path);
            
        let label = format!("Match in {} at line {}", file_name, start_line);
        
        matches.push(MatchItem {
            file_path,
            line: start_line,
            column: start_col,
            label,
            snippet,
        });
    }
    
    Ok(matches)
}

fn print_help() {
    println!(
        "gluek-up: Spatial code navigation peek utility.

Usage:
  gluek-up [MODE]

Modes:
  -d, --definitions    Peek definitions for symbol at cursor
  -r, --references     Peek references for symbol at cursor
  -h, --help           Show this help"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        std::process::exit(0);
    }
    
    let mut is_refs = false;
    let mut is_defs = false;
    
    for arg in args.iter().skip(1) {
        if arg == "--definitions" || arg == "-d" {
            is_defs = true;
        } else if arg == "--references" || arg == "-r" {
            is_refs = true;
        }
    }
    
    if !is_refs && !is_defs {
        is_defs = true;
    }
    
    let context = fetch_context().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to connect to Current socket: {}", e);
        ContextResponse {
            app_id: Some("mock-editor".to_string()),
            pid: Some(9999),
            attention: Some(Attention {
                file: "/media/veracrypt1/_PROJECTS/_Future/Current/src/main.rs".to_string(),
                selections: vec![Selection {
                    line: 10,
                    column: 8,
                    anchor_line: 10,
                    anchor_column: 8,
                }],
            }),
        }
    });
    
    let active_file = context.attention.as_ref()
        .map(|att| att.file.clone())
        .unwrap_or_else(|| "/media/veracrypt1/_PROJECTS/_Future/Current/src/main.rs".to_string());
        
    let line = context.attention.as_ref()
        .and_then(|att| att.selections.first())
        .map(|s| s.line)
        .unwrap_or(1);
        
    let col = context.attention.as_ref()
        .and_then(|att| att.selections.first())
        .map(|s| s.column)
        .unwrap_or(1);
    
    // Attempt to query real lsp-broker, fallback to mocks on failure
    let matches = fetch_broker_matches(is_defs, &active_file, line, col).unwrap_or_else(|e| {
        eprintln!("Info: Query socket query failed (using Stage-1 mock fallback): {}", e);
        if is_defs {
            vec![
                MatchItem {
                    file_path: active_file.clone(),
                    line: 10,
                    column: 8,
                    label: "struct Selection (Mock Definition)".to_string(),
                    snippet: "struct Selection {".to_string(),
                },
                MatchItem {
                    file_path: "/media/veracrypt1/_PROJECTS/_Future/TO-DAY/src/ui.rs".to_string(),
                    line: 10,
                    column: 8,
                    label: "pub fn activate (Mock Alt Definition)".to_string(),
                    snippet: "pub fn activate(application: &gtk::Application, state: AppState) {".to_string(),
                },
            ]
        } else {
            vec![
                MatchItem {
                    file_path: active_file.clone(),
                    line: 10,
                    column: 8,
                    label: "Selection definition (Mock)".to_string(),
                    snippet: "struct Selection {".to_string(),
                },
                MatchItem {
                    file_path: active_file.clone(),
                    line: 20,
                    column: 20,
                    label: "Selection field in Attention (Mock)".to_string(),
                    snippet: "    selections: Vec<Selection>,".to_string(),
                },
                MatchItem {
                    file_path: "/media/veracrypt1/_PROJECTS/_Future/Current/src/main.rs".to_string(),
                    line: 10,
                    column: 8,
                    label: "Selection struct definition in Current (Mock)".to_string(),
                    snippet: "struct Selection {".to_string(),
                },
                MatchItem {
                    file_path: "/media/veracrypt1/_PROJECTS/_Future/TO-DAY/src/ui.rs".to_string(),
                    line: 5,
                    column: 16,
                    label: "use crate::AppState reference (Mock)".to_string(),
                    snippet: "use crate::AppState;".to_string(),
                },
            ]
        }
    });
    
    ui::run(matches, is_refs);
}
