extern crate pretty_env_logger;
#[macro_use] extern crate log;
mod event_handler;
use std::borrow::Cow;
use std::collections::HashMap;
use std::process;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use reqwest::Url;
use tungstenite::{connect, Message};
use clap::Parser;
use log::LevelFilter;
use reqwest::blocking::Client;
use tungstenite::protocol::CloseFrame;
use tungstenite::protocol::frame::coding::CloseCode;

#[derive(Deserialize)]
struct WsUrlResponse {
    ok: bool,
    url: String,
}

#[derive(Serialize)]
struct EventSubscribeRequest {
    id: u64,
    r#type: String,
    event: String
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, env)]
    private_key: String,

    #[arg(short, long, env)]
    reconnect: bool,

    #[arg(short, long, env)]
    sync_node: Option<String>
}

fn main() {
    let cli = Cli::parse();
    let sync_node = cli.sync_node.unwrap_or("https://tenebra.lil.gay".to_string());
    let http_client = Client::new();
    pretty_env_logger::formatted_builder().filter_level(LevelFilter::Debug).init();

    info!("TenebraStake by Erb3. Reconnecting is {}", if cli.reconnect { "enabled" } else { "disabled" });

    let ws_url_data = get_ws_url(cli.private_key, sync_node.clone(), &http_client);

    if !ws_url_data.ok {
        error!("Unable to fetch WebSocket URL");
        process::exit(1);
    }

    info!("WS Url: {}", ws_url_data.url);
    let (socket, _) =
        connect(Url::parse(&ws_url_data.url).unwrap()).expect("Unable to connect to WebSocket");

    let socket = Arc::new(Mutex::new(socket));
    let socket_signals = Arc::clone(&socket);
    let socket_replies = Arc::clone(&socket);
    let mut next_id: u64 = 1;

    ctrlc::set_handler(move || {
        info!("Received SIGTERM, shutting down.");

        let mut socket = socket_signals.lock().unwrap();
        debug!("Un-mutexed socket");

        let close_frame = CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::from("CTRL+C")
        };

        socket.write(Message::Close(Some(close_frame))).expect("Expected to be able to close websocket lol");
        debug!("Sent close packet");
    }).expect("Exception while setting CTRL + C handler");

    info!("Connected to WebSocket server");

    let subscribe_packet = EventSubscribeRequest {
        id: 0,
        r#type: "subscribe".to_string(),
        event: "ownValidators".to_string(),
    };
    socket.lock().unwrap().send(
        Message::Text(serde_json::to_string(&subscribe_packet).unwrap())
    ).expect("Exception while sending subscription packet");

    loop {
        let msg = socket.lock().unwrap().read().expect("Error reading WebSocket message");

        match msg {
            Message::Close(close_frame) => {

                if close_frame.clone().unwrap().reason == "CTRL+C" {
                    info!("WebSocket gracefully closed due to SIGTERM");
                } else {
                    warn!("WebSocket abruptly closed. {:?}", close_frame);
                }

                process::exit(0);
            }

            Message::Text(txt) => {
                event_handler::on_msg(serde_json::from_str(&txt).expect("Expect websocket message to be json"), socket_replies.clone(), &mut next_id);
            }

            _ => {}
        }
    }
}

fn get_ws_url(private_key: String, sync_node: String, http_client: &Client) -> WsUrlResponse {
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert("privatekey".to_string(), private_key);

    let res = http_client.post(sync_node + "/ws/start").json(&body).send().expect("Expected sync node to be online");
    res.json().expect("Expected JSON response with websocket URL. Is the sync node correct?")
}