use clap::Parser;
use futures_util::{future, pin_mut, StreamExt};
use log::{debug, error, info, warn, LevelFilter};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, process};
use tokio::signal;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[derive(Deserialize)]
struct WsUrlResponse {
    ok: bool,
    url: String,
}

#[derive(Serialize)]
struct EventSubscribeRequest {
    id: u32,
    #[serde(rename = "type")]
    event_type: String,
    event: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, env)]
    private_key: String,

    #[arg(short, long, env)]
    sync_node: Option<String>,
}

#[derive(Debug, Serialize)]
struct BlockSubmit {
    id: u32,
    #[serde(rename = "type")]
    event_type: String,
    nonce: String,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    address: String,
    value: u32,
}

#[derive(Deserialize)]
struct StakeUpdatePacket {
    active: bool,
    #[serde(rename = "owner")]
    address: String,
    #[serde(rename = "stake")]
    new_amount: u32,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let sync_node = cli
        .sync_node
        .unwrap_or("https://tenebra.lil.gay".to_string());
    let http_client = Client::new();
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Debug)
        .init();

    info!("TenebraStake made by Erb3. https://github.com/Erb3/TenebraStake");

    let submit_block = Message::Text(
        serde_json::to_string(&BlockSubmit {
            id: 69420,
            event_type: "submit_block".to_string(),
            nonce: "erb3tenebrastake".to_string(), // Tenebra :skull:
        })
        .unwrap(),
    );

    let ws_url_data = get_ws_url(cli.private_key, sync_node.clone(), &http_client).await;

    if !ws_url_data.ok {
        error!("Unable to fetch WebSocket URL");
        process::exit(1);
    }

    info!("WS Url: {}", ws_url_data.url);
    let (ws_stream, _) = connect_async(&Url::parse(&ws_url_data.url).unwrap())
        .await
        .expect("Failed to establish connection with host WebSocket");
    let (ws_write, ws_read) = ws_stream.split();
    let (tx, rx) = futures_channel::mpsc::unbounded();
    let to_ws = rx.map(Ok).forward(ws_write);

    info!("Connected to WebSocket server");

    let subscribe_packet = EventSubscribeRequest {
        id: 0,
        event_type: "subscribe".to_string(),
        event: "ownValidators".to_string(),
    };
    tx.unbounded_send(Message::Text(
        serde_json::to_string(&subscribe_packet).unwrap(),
    ))
    .expect("Exception while sending subscription packet");

    tokio::spawn(async move {
        let from_ws = {
            ws_read.for_each(|message| async {
                match message.unwrap_or(Message::Close(None)) {
                    Message::Close(_v) => {
                        error!("Websocket closed!");
                        process::exit(0);
                    }

                    Message::Text(txt) => {
                        let data: Value = serde_json::from_str(&txt)
                            .expect("Expect websocket message to be json");

                        if data["id"].is_number() {
                            info!("Ignoring response for id {}: {:?}", data["id"], data)
                        } else {
                            match data["type"].as_str().unwrap() {
                                "keepalive" => {}

                                "hello" => {
                                    info!("Tenebra server says hello 👋 {:?}", data);
                                }

                                "event" => match data["event"].as_str().unwrap() {
                                    "transaction" => {}

                                    "block" => {
                                        let block: Block =
                                            serde_json::from_str(&data["block"].to_string())
                                                .unwrap();
                                        info!(
                                            "{} just earned t{} from staking.",
                                            block.address, block.value
                                        );
                                    }

                                    "stake" => {
                                        let stake: StakeUpdatePacket =
                                            serde_json::from_str(&data["stake"].to_string())
                                                .unwrap();
                                        info!(
                                            "{} just updated their {}stake to {}",
                                            stake.address,
                                            if stake.active { "" } else { "inactive " },
                                            stake.new_amount
                                        )
                                    }

                                    "validator" => {
                                        debug!("Staking packet {:?}", data);

                                        let to_submit = submit_block.clone();
                                        info!("Submitting block {:?}", to_submit);
                                        tx.unbounded_send(to_submit)
                                            .expect("Exception while submitting block");
                                    }

                                    _ => {
                                        warn!("Unexpected event {:?}", data);
                                    }
                                },
                                _ => {
                                    warn!("Unrecognized packet! {}", data);
                                }
                            }
                        }
                    }

                    _ => {}
                }
            })
        };

        pin_mut!(to_ws, from_ws);
        future::select(to_ws, from_ws).await;
    });

    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Received SIGTERM, exiting!");
            process::exit(0);
        }
        Err(err) => {
            panic!("Unable to listen for shutdown signal: {}", err);
        }
    }
}

async fn get_ws_url(private_key: String, sync_node: String, http_client: &Client) -> WsUrlResponse {
    let mut body: HashMap<String, String> = HashMap::new();
    body.insert("privatekey".to_string(), private_key);

    let res = http_client
        .post(sync_node + "/ws/start")
        .json(&body)
        .send()
        .await
        .expect("Expected sync node to be online");
    res.json()
        .await
        .expect("Expected JSON response with websocket URL. Is the sync node correct?")
}
