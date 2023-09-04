use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use base64::Engine;
use rand::random;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

#[derive(Debug, Serialize)]
struct BlockSubmit {
    id: u32,
    r#type: String,
    nonce: String
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    address: String,
    value: u32
}

pub fn on_msg(data: Value, socket: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>, next_id: &mut u32) {
    if data["id"].is_number() {
        info!("Received response for id {}: {:?}", data["id"], data)
    } else {
        match data["type"].as_str().unwrap() {
            "keepalive" => {},
            "hello" => {
                info!("Tenebra server says hello 👋 {:?}", data);
            },
            "event" => {
                match data["event"].as_str().unwrap() {
                    "block" => {
                        let block: Block = serde_json::from_str(&data["block"].to_string()).unwrap();
                        info!("{} just earned t{} from staking.", block.address, block.value);
                    },

                    "validator" => {
                        debug!("Staking packet {:?}", data);

                        let submit_block = BlockSubmit {
                            id: *next_id,
                            r#type: "submit_block".to_string(),
                            nonce: base64::engine::general_purpose::STANDARD.encode(random::<[u8; 16]>()),
                        };

                        *next_id += 1;

                        info!("Submitting block {:?}", submit_block);
                        socket.lock().unwrap()
                            .send(Message::Text(serde_json::to_string(&submit_block).unwrap())).expect("Exception while submitting block");
                    },

                    _ => {
                        warn!("Unexpected event {:?}", data);
                    }
                }
            },
            _ => {
                warn!("Unrecognized packet! {}", data);
            }
        }
    }
}