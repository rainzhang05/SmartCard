use std::io::{self, Read, Write};
use serde::{Deserialize, Serialize};
use log::{info, error};

#[derive(Serialize, Deserialize, Debug)]
struct NativeMessage {
    text: String,
}

fn main() {
    // Initialize logger (optional, might need a file logger since stdout is used for messaging)
    // env_logger::init();

    loop {
        match read_input() {
            Ok(input) => {
                // Process input
                let response = NativeMessage {
                    text: format!("Echo: {}", input.text),
                };
                if let Err(e) = write_output(&response) {
                    // error!("Failed to write output: {}", e);
                    break;
                }
            }
            Err(e) => {
                // error!("Failed to read input: {}", e);
                break;
            }
        }
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

fn write_output(message: &NativeMessage) -> io::Result<()> {
    let json = serde_json::to_vec(message)?;
    let length = json.len() as u32;
    let length_bytes = length.to_ne_bytes();

    io::stdout().write_all(&length_bytes)?;
    io::stdout().write_all(&json)?;
    io::stdout().flush()?;
    Ok(())
}
