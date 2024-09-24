use std::sync::Arc;

use parking_lot::Mutex;
use rocket::futures::channel::mpsc::{channel, Sender};
use rocket::futures::{SinkExt, StreamExt};
//use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::select;
//use rocket::tokio::sync::Mutex;
use rocket::{get, State};
use rocket_ws::Message;
use rocket_ws::{Channel, WebSocket};

use crate::bus::BusMap;
use crate::game::engine::Engine;

// TODO redo
//type PeersMap = Arc<Mutex<HashMap<String, Sender<String>>>>;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct KeyMessage {
    key: u8,
    event: String,
}

//type EngineState = Arc<Mutex<Engine>>;

#[get("/ws/key-handler")]
pub async fn key_handler(ws: WebSocket, bus_map: &State<BusMap<String>>) -> Channel<'static> {
    let bus_map = bus_map.inner().clone();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                select! {
                    message = stream.next() => match message {
                        Some(Ok(Message::Text(text))) => {
                            println!("Received message: {:?}", text);

                            //let msg: KeyMessage = serde_json::from_str(&text).unwrap();
                            let mut bp = bus_map.lock().await;
                            let bus = bp.get_mut(&1).unwrap();

                            bus.broadcast(text);
                        }
                        Some(Ok(message)) => {
                            println!("Received message from client: {:?}", message);
                            let _ = stream.send(message).await;
                        }
                        Some(Err(error)) => {
                            println!("Error: {:?}", error);
                            break;
                        }
                        None => break,
                    },
                    else => break,
                }
            }

            Ok(())
        })
    })
}

// loop
// rx.try_recv
// if yes => ws.send
