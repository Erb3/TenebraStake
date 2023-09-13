use std::net::TcpStream;
use std::sync::{Arc, Mutex};
// use rand::Rng;
// use rand::distributions::Alphanumeric;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

#[derive(Debug, Serialize)]
struct BlockSubmit {
    id: u32,
    #[serde(rename = "type")]
    event_type: String,
    nonce: String
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    address: String,
    value: u32
}

#[derive(Deserialize)]
struct StakeUpdatePacket {
    active: bool,
    #[serde(rename = "owner")]
    address: String,
    #[serde(rename = "stake")]
    new_amount: u32
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
                    "transaction" => {},

                    "block" => {
                        let block: Block = serde_json::from_str(&data["block"].to_string()).unwrap();
                        info!("{} just earned t{} from staking.", block.address, block.value);
                    },

                    "stake" => {
                        let stake: StakeUpdatePacket = serde_json::from_str(&data["stake"].to_string()).unwrap();
                        info!("{} just updated their {}stake to {}", stake.address, if stake.active {""} else {"inactive "}, stake.new_amount)
                    },

                    "validator" => {
                        debug!("Staking packet {:?}", data);

                        // let nonce: String = rand::thread_rng().sample_iter(&Alphanumeric).take(16).map(char::from).collect();
                        let submit_block = BlockSubmit {
                            id: *next_id,
                            event_type: "submit_block".to_string(),
                            nonce: "erb3tenebrastake".to_string(), // Tenebra :skull:
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
