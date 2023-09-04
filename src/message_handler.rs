use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::WebSocket;
use crate::staker::stake;

#[derive(Serialize, Deserialize)]
pub struct Block {
    address: String,
    difficulty: u64,
    hash: String,
    height: u64,
    pub(crate) short_hash: String,
    time: String,
    value: u32
}

pub fn on_msg(data: Value, current_hash: &mut String, socket: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>) {

    if !data["id"].is_null() {
        info!("Received response for id {}: {:?}", data["id"], data)
    } else {
        match data["type"].as_str().unwrap() {
            "keepalive" => {
                debug!("Received keep-alive packet")
            },
            "hello" => {
                info!("Tenebra server says hello 👋 {:?}", data);
            },
            "event" => {
                match data["event"].as_str().unwrap() {
                    "block" => {
                        let block: Block = serde_json::from_str(&data["block"].to_string()).unwrap();
                        info!("{} just earned t{} from staking.", block.address, block.value);
                        *current_hash = block.short_hash;
                        debug!("Hash is now {}", current_hash);
                    },

                    "validator" => {
                        debug!("Staking packet {:?}", data);
                        stake(current_hash.to_string(), socket);
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