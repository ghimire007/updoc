use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    extract::State,
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use redis::{Commands, RedisResult};
use crate::config::app_state::AppState;
use serde::{Deserialize, Serialize};

use crate::config::config::get_env;
use crate::config::db::{PUBLISHER,WSH};
use crate::config::jwt::create_token;
use crate::error::AppError;
use crate::models::users::{self, Entity as User,Model,ActiveModel};
use crate::schemas::auth::{login::LoginRequest, signup::SignupRequest};
use crate::schemas::ws::Wmsg;
use crate::schemas::response::{SucessResponse, TokenResponse};
use axum::{Json, Extension,extract::Path};
use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::{entity::*, query::*, DbBackend};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::result::Result;
use std::time::Duration;
use validator::Validate;
//use crate::config::db::r_handler;
use std::{net::SocketAddr, sync::{Mutex,Arc}};
use futures::{stream::SplitSink, SinkExt, StreamExt};



pub async fn handler( 
    State(state): State<AppState>,
    Path(id): Path<String>,
    ws: WebSocketUpgrade) -> Response {

    ws.on_upgrade(move |socket| handle_socket(socket, id,State(state)))
    
}

async fn handle_socket(socket: WebSocket, id:String, State(state): State<AppState>) {
    println!("whats the reason");
    let (sender, mut receiver) = socket.split();
    
    println!("whats the reason 2");
    let x=WSH.clone();
    let mut man= &mut *x.lock().unwrap();
    println!("whats the reason 3");
    man.add_connection(id, 64, sender) ;
    println!("whats the reason 4");

    tokio::spawn(async move {
        

        while let Some(msg) = receiver.next().await {
            let msg = if let Ok(msg) = msg {
                msg
            } else {
                // client disconnected
                return;
            };
            println!("some message received");
            let m=msg.clone();
            let parsed_message: Wmsg =  serde_json::from_str::<Wmsg>(
                &m.into_text().unwrap()
            ).unwrap();
            
            match (parsed_message.key.as_str()) {
                "relay" => {
                    let json_message = serde_json::to_string(&parsed_message).expect("Failed to serialize to JSON");
        
                    // Clone the Arc and lock the Mutex to access the Connection.
                    let publisher = &*PUBLISHER.clone();
                    let mut publisher = publisher.lock().unwrap();
                    let _: RedisResult<()> = publisher.publish("messages", &json_message);
                    println!("Published")
                },
                _=>{
                    println!("heelo");
            }
                
            }
            // if socket.send(msg).await.is_err() {
            //     // client disconnected
            //     return;
            }
         

     
    });


    }
        
       
        
    
