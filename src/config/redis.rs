use crate::config::db::WSH;
use crate::schemas::ws::Wmsg;
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use redis::{Commands, ControlFlow, PubSub};
use std::sync::{Arc, Mutex as MU};
use std::{error::Error, thread};
use tokio;
use tokio::sync::Mutex;
//use crate::managers::websocketmanager::GlobalWebSocketManager;

pub fn connect() -> redis::Connection {
    let redis_conn_url = "redis://127.0.0.1:6379";
    //format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    //println!("{}", redis_conn_url);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .map_err(|err| println!("{:?}", err))
        .expect("failed to connect to Redis")
}

pub fn run_subscriber() -> Result<(), Box<dyn Error>> {
    println!("reached_here");

    let wsh = WSH.clone();

    thread::spawn(move || {
        // Your existing code for setting up the connection and subscription

        // Inside the spawned thread, use Tokio's runtime
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let mut conn = connect();
            let mut n_connection = conn.as_pubsub();
            n_connection.subscribe("messages");
            let wsh = WSH.clone();

            loop {
                let msg: redis::Msg = n_connection.get_message().unwrap();
                let payload: String = msg.get_payload().unwrap();

                let parsed_message: Wmsg =
                    serde_json::from_str::<Wmsg>(&payload.to_string()).unwrap();
                match (parsed_message.key.as_str()) {
                    "relay" => {
                        println!("got a message");
                        let mut ws = &mut *WSH.lock().unwrap();
                        println!("reacged here subs msg is :: {:?}", parsed_message);
                        ws.broadcast(parsed_message).await;
                    }
                    _ => {
                        println!("{:?}", parsed_message.key);
                    }
                }
            }
        });
    });

    Ok(())
}
