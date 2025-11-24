use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use log::{info, error};
use pcsc::{Context, Scope, Card, Protocols, ShareMode};
use std::ffi::CString;

#[derive(Serialize, Deserialize, Debug)]
struct NativeMessage {
    i: String, // Message ID
    c: String, // Command: "list_readers", "connect", "transceive"
    p: Option<serde_json::Value>, // Parameters
}

#[derive(Serialize, Deserialize, Debug)]
struct NativeResponse {
    i: String,
    s: String, // Status: "success", "error"
    d: Option<serde_json::Value>, // Data
    e: Option<String>, // Error message
}

struct AppState {
    context: Option<Context>,
    card: Option<Card>,
}

fn main() {
    // Initialize logger (file logger recommended for native messaging)
    // env_logger::init();

    let app_state = Arc::new(Mutex::new(AppState {
        context: Context::establish(Scope::User).ok(),
        card: None,
    }));

    loop {
        match read_input() {
            Ok(input) => {
                let response = handle_message(&input, &app_state);
                if let Err(_) = write_output(&response) {
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn handle_message(msg: &NativeMessage, state: &Arc<Mutex<AppState>>) -> NativeResponse {
    let mut state = state.lock().unwrap();
    
    match msg.c.as_str() {
        "list_readers" => {
            if state.context.is_none() {
                state.context = Context::establish(Scope::User).ok();
            }

            match &state.context {
                Some(ctx) => {
                    match ctx.list_readers_len() {
                        Ok(len) => {
                            let mut readers_buf = vec![0; len];
                            match ctx.list_readers(&mut readers_buf) {
                                Ok(readers) => {
                                    let reader_names: Vec<String> = readers
                                        .map(|r| r.to_string_lossy().into_owned())
                                        .collect();
                                    NativeResponse {
                                        i: msg.i.clone(),
                                        s: "success".to_string(),
                                        d: Some(serde_json::to_value(reader_names).unwrap()),
                                        e: None,
                                    }
                                }
                                Err(e) => error_response(&msg.i, &format!("Failed to list readers: {}", e)),
                            }
                        }
                        Err(e) => error_response(&msg.i, &format!("Failed to get readers length: {}", e)),
                    }
                }
                None => error_response(&msg.i, "Failed to establish PC/SC context"),
            }
        }
        "connect" => {
            let reader_name = msg.p.as_ref()
                .and_then(|p| p.get("reader"))
                .and_then(|r| r.as_str());

            match (reader_name, &state.context) {
                (Some(name), Some(ctx)) => {
                    match CString::new(name) {
                        Ok(cname) => {
                            match ctx.connect(&cname, ShareMode::Shared, Protocols::ANY) {
                                Ok(card) => {
                                    state.card = Some(card);
                                    NativeResponse {
                                        i: msg.i.clone(),
                                        s: "success".to_string(),
                                        d: Some(serde_json::json!({"connected": true})),
                                        e: None,
                                    }
                                }
                                Err(e) => error_response(&msg.i, &format!("Failed to connect: {}", e)),
                            }
                        }
                        Err(_) => error_response(&msg.i, "Invalid reader name"),
                    }
                }
                (None, _) => error_response(&msg.i, "Missing reader name"),
                (_, None) => error_response(&msg.i, "No PC/SC context"),
            }
        }
        "transceive" => {
            let apdu_hex = msg.p.as_ref()
                .and_then(|p| p.get("apdu"))
                .and_then(|a| a.as_str());

            match (apdu_hex, &state.card) {
                (Some(hex_str), Some(card)) => {
                    match hex::decode(hex_str) {
                        Ok(apdu) => {
                            let mut rapdu_buf = [0; 4096]; // Max APDU size
                            match card.transmit(&apdu, &mut rapdu_buf) {
                                Ok(rapdu) => {
                                    NativeResponse {
                                        i: msg.i.clone(),
                                        s: "success".to_string(),
                                        d: Some(serde_json::json!({
                                            "rapdu": hex::encode(rapdu)
                                        })),
                                        e: None,
                                    }
                                }
                                Err(e) => error_response(&msg.i, &format!("Transmit failed: {}", e)),
                            }
                        }
                        Err(_) => error_response(&msg.i, "Invalid APDU hex"),
                    }
                }
                (None, _) => error_response(&msg.i, "Missing APDU"),
                (_, None) => error_response(&msg.i, "No card connected"),
            }
        }
        _ => error_response(&msg.i, "Unknown command"),
    }
}

fn error_response(id: &str, msg: &str) -> NativeResponse {
    NativeResponse {
        i: id.to_string(),
        s: "error".to_string(),
        d: None,
        e: Some(msg.to_string()),
    }
}

fn read_input() -> io::Result<NativeMessage> {
    let mut length_bytes = [0u8; 4];
    io::stdin().read_exact(&mut length_bytes)?;
    let length = u32::from_ne_bytes(length_bytes) as usize;

    let mut buffer = vec![0u8; length];
    io::stdin().read_exact(&mut buffer)?;

    let message: NativeMessage = serde_json::from_slice(&buffer)?;
    Ok(message)
}

fn write_output(message: &NativeResponse) -> io::Result<()> {
    let json = serde_json::to_vec(message)?;
    let length = json.len() as u32;
    let length_bytes = length.to_ne_bytes();

    io::stdout().write_all(&length_bytes)?;
    io::stdout().write_all(&json)?;
    io::stdout().flush()?;
    Ok(())
}
