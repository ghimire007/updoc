use axum::{
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use redis::{Commands, RedisResult};

use std::mem;

use crate::config::config::get_env;
use crate::config::db::DB;
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
use std::{collections::BTreeMap, thread, error::Error};
use std::convert::TryInto;
use std::result::Result;
use std::time::Duration;
use validator::Validate;
use crate::config::db::{PUBLISHER,WSH};
//use crate::config::db::r_handler;



pub struct UnitSocket{
    pub user_id: i64,
    pub id:i64,
    pub connection : SplitSink<WebSocket,Message>

    
}

impl UnitSocket{
    pub fn new(user_id:i64,id:i64,connection:SplitSink<WebSocket,Message>)->UnitSocket{
        return UnitSocket{
            user_id:user_id,
            id:id,
            connection: connection
        }
    }

    pub fn close(self){
        todo!();
        }

    // pub async fn listen(&mut self){
    //     println!("reached_here 45346");

    //     while let Some(msg) = &self.connection.recv().await {
    //         let msg = if let Ok(msg) = msg {
    //             msg
    //         } else {
    //             // client disconnected
    //             return;
    //         };
    //         let m=msg.clone();
    //         let parsed_message: Wmsg =  serde_json::from_str::<Wmsg>(
    //             &m.into_text().unwrap()
    //         ).unwrap();
            
    //         match (parsed_message.key.as_str()) {
    //             "relay" => {
    //                 let json_message = serde_json::to_string(&parsed_message).expect("Failed to serialize to JSON");
        
    //                 // Clone the Arc and lock the Mutex to access the Connection.
    //                 let publisher = &*PUBLISHER.clone();
    //                 let mut publisher = publisher.lock().unwrap();
    //                 let _: RedisResult<()> = publisher.publish("messages", &json_message);
    //             },
    //             _=>{
    //                 println!("heelo");
    //         }
                
    //         }
    //         // if socket.send(msg).await.is_err() {
    //         //     // client disconnected
    //         //     return;
    //         }
    // }
    

    


}