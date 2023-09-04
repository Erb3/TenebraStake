use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use base64::Engine;
use rand::random;
use serde::Serialize;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

#[derive(Debug)]
#[derive(Serialize)]
struct BlockSubmit {
    id: u64,
    r#type: String,
    nonce: String
}

pub fn stake(current_hash: String, socket: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>) {
    // let nonce = rand::thread_rng().gen::<[u8; 16]>();
    let nonce = random::<[u8; 16]>();
    let encoded_nonce = base64::engine::general_purpose::STANDARD.encode(nonce);
    // let hash_string = encoded_nonce.clone() + "";
    // let hash = sha256::digest(hash_string);

    let submit_block = BlockSubmit {
        id: 0,
        r#type: "submit_block".to_string(),
        nonce: encoded_nonce,
    };

    info!("Submitting block {:?}", submit_block);
    socket.lock().unwrap()
        .send(Message::Text(serde_json::to_string(&submit_block).unwrap())).expect("Exception while submitting block");
}