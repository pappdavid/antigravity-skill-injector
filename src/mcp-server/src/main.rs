use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Write};

#[derive(Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Value,
    result: Value,
}

#[derive(Serialize, Deserialize)]
struct JsonRpcError {
    jsonrpc: String,
    id: Value,
    error: Value,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let registry_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "/Users/davidpapp/.gemini/antigravity/scratch/skill_injection_architecture/skills_registry.json".to_string()
    };
    
    let skills_data = match fs::read_to_string(&registry_path) {
        Ok(data) => data,
        Err(_) => "{}".to_string(),
    };
    
    let skills: HashMap<String, Value> = serde_json::from_str(&skills_data).unwrap_or_default();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break, // EOF or error
        };

        if line.trim().is_empty() {
            continue;
        }

        let req: Result<JsonRpcRequest, _> = serde_json::from_str(&line);
        if let Ok(request) = req {
            // Handle notifications
            if request.id.is_none() {
                continue;
            }
            let req_id = request.id.unwrap();

            match request.method.as_str() {
                "initialize" => {
                    let result = json!({
                        "protocolVersion": "2024-11-05",
                        "capabilities": {
                            "tools": {}
                        },
                        "serverInfo": {
                            "name": "mcp-skill-injector",
                            "version": "1.0.0"
                        }
                    });
                    send_response(&mut stdout, req_id, result);
                }
                "tools/list" => {
                    let mut tool_list = Vec::new();
                    tool_list.push(json!({
                        "name": "list_skills",
                        "description": "Returns a list of all available Antigravity skills and their descriptions.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    }));
                    tool_list.push(json!({
                        "name": "load_skill",
                        "description": "Returns the full text instructions and required tool schemas for a specific Antigravity skill.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "skill_id": {
                                    "type": "string",
                                    "description": "The ID of the skill to load"
                                }
                            },
                            "required": ["skill_id"]
                        }
                    }));
                    let result = json!({ "tools": tool_list });
                    send_response(&mut stdout, req_id, result);
                }
                "tools/call" => {
                    if let Some(params) = request.params {
                        let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                        let arguments = params.get("arguments");

                        if name == "list_skills" {
                            let mut text = String::from("Available Skills:\n");
                            for (id, data) in &skills {
                                let desc = data.get("description").and_then(|v| v.as_str()).unwrap_or("");
                                text.push_str(&format!("- **{}**: {}\n", id, desc));
                            }
                            let result = json!({
                                "content": [{
                                    "type": "text",
                                    "text": text
                                }]
                            });
                            send_response(&mut stdout, req_id, result);
                        } else if name == "load_skill" {
                            let skill_id = arguments
                                .and_then(|args| args.get("skill_id"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("");

                            if let Some(skill_data) = skills.get(skill_id) {
                                let mut text = format!("Skill Loaded: {}\n\n", skill_id);
                                if let Some(instructions) = skill_data.get("instructions").and_then(|v| v.as_str()) {
                                    text.push_str("Instructions:\n");
                                    text.push_str(instructions);
                                    text.push_str("\n\n");
                                }
                                if let Some(schema) = skill_data.get("schema") {
                                    text.push_str("Tool Schema:\n");
                                    text.push_str(&serde_json::to_string_pretty(schema).unwrap_or_default());
                                    text.push_str("\n");
                                }

                                let result = json!({
                                    "content": [{
                                        "type": "text",
                                        "text": text
                                    }]
                                });
                                send_response(&mut stdout, req_id, result);
                            } else {
                                let result = json!({
                                    "content": [{
                                        "type": "text",
                                        "text": format!("Skill '{}' not found.", skill_id)
                                    }],
                                    "isError": true
                                });
                                send_response(&mut stdout, req_id, result);
                            }
                        } else {
                            send_error(&mut stdout, req_id, -32601, "Method not found");
                        }
                    } else {
                        send_error(&mut stdout, req_id, -32602, "Invalid params");
                    }
                }
                _ => {
                    send_error(&mut stdout, req_id, -32601, "Method not found");
                }
            }
        }
    }
}

fn send_response(stdout: &mut io::Stdout, id: Value, result: Value) {
    let res = JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id,
        result,
    };
    let json_str = serde_json::to_string(&res).unwrap();
    writeln!(stdout, "{}", json_str).unwrap();
    stdout.flush().unwrap();
}

fn send_error(stdout: &mut io::Stdout, id: Value, code: i32, message: &str) {
    let err = JsonRpcError {
        jsonrpc: "2.0".to_string(),
        id,
        error: json!({ "code": code, "message": message }),
    };
    let json_str = serde_json::to_string(&err).unwrap();
    writeln!(stdout, "{}", json_str).unwrap();
    stdout.flush().unwrap();
}
