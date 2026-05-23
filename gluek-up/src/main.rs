use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use serde::{Deserialize, Serialize};

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

fn fetch_context() -> Result<ContextResponse, Box<dyn std::error::Error>> {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| "/run/user/1000".to_string());
    let socket_path = Path::new(&runtime_dir).join("current.sock");
    
    let mut stream = UnixStream::connect(socket_path)?;
    let query_req = serde_json::json!({ "type": "Query" });
    stream.write_all(format!("{}\n", query_req).as_bytes())?;
    
    let mut response_buf = String::new();
    stream.read_to_string(&mut response_buf)?;
    
    let context: ContextResponse = serde_json::from_str(&response_buf)?;
    Ok(context)
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
    
    // Default to definitions if unspecified
    if !is_refs && !is_defs {
        is_defs = true;
    }
    
    // Fetch context from Current daemon
    let context = fetch_context().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to connect to Current socket: {}", e);
        // Fallback mock context using existing project files
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
    
    // Generate Stage-1 mock matches
    let matches = if is_defs {
        vec![
            MatchItem {
                file_path: active_file.clone(),
                line: 10,
                column: 8,
                label: "struct Selection (Definition)".to_string(),
                snippet: "struct Selection {".to_string(),
            },
            MatchItem {
                file_path: "/media/veracrypt1/_PROJECTS/_Future/TO-DAY/src/ui.rs".to_string(),
                line: 10,
                column: 8,
                label: "pub fn activate (Alt Definition)".to_string(),
                snippet: "pub fn activate(application: &gtk::Application, state: AppState) {".to_string(),
            },
        ]
    } else {
        // References mode
        vec![
            MatchItem {
                file_path: active_file.clone(),
                line: 10,
                column: 8,
                label: "Selection definition".to_string(),
                snippet: "struct Selection {".to_string(),
            },
            MatchItem {
                file_path: active_file.clone(),
                line: 20,
                column: 20,
                label: "Selection field in Attention".to_string(),
                snippet: "    selections: Vec<Selection>,".to_string(),
            },
            MatchItem {
                file_path: "/media/veracrypt1/_PROJECTS/_Future/Current/src/main.rs".to_string(),
                line: 10,
                column: 8,
                label: "Selection struct definition in Current".to_string(),
                snippet: "struct Selection {".to_string(),
            },
            MatchItem {
                file_path: "/media/veracrypt1/_PROJECTS/_Future/TO-DAY/src/ui.rs".to_string(),
                line: 5,
                column: 16,
                label: "use crate::AppState reference".to_string(),
                snippet: "use crate::AppState;".to_string(),
            },
        ]
    };
    
    // Run the GTK UI with the resolved matches
    ui::run(matches, is_refs);
}
