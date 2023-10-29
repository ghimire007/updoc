use redis::{Commands,ControlFlow, PubSub};
use crate::schemas::ws::Wmsg;
use std::error::Error;
use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    extract::State,
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use std::{net::SocketAddr, sync::{Mutex,Arc}};
use crate::managers::websocketmanager::GlobalWebSocketManager;


use super::app_state::AppState;

pub fn connect() -> redis::Connection {


    let redis_conn_url = "redis://127.0.0.1:6379";
    //format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    //println!("{}", redis_conn_url);

    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection().map_err(|err|{
            println!("{:?}",err)
        }).expect("failed to connect to Redis")
        
}




pub struct RedisHandler{
    pub connection:redis::Connection,


}

impl RedisHandler{

    pub fn new()->RedisHandler{
        let mut redis=connect();

        RedisHandler { connection: redis }

        
        }

    pub fn publish(&mut self,file_id:String,msg:Wmsg)->Result<(),Box<dyn Error>>{

        let json_message= serde_json::to_string(&msg)?;
        self.connection.publish(file_id,json_message)?;
        println!("published");
        Ok(())

    }

    // pub async fn subscribe(&mut self,file_id:String,wsh:Arc<Mutex<GlobalWebSocketManager>>)->Result<(),Box<dyn Error>>{
    //     //let mut pub_sub= self.connection.as_pubsub();
        
    //     println!("reacged here subs");
    //     tokio::spawn(async move {
    //     let mut  conn= connect();
    //     let mut pub_sub=conn.as_pubsub();
    //     pub_sub.subscribe("messages").expect("error");
        
    //    loop{
    //     let msg: redis::Msg=pub_sub.get_message().unwrap();
    //     println!("message {:?}",msg);
    //     println!("at lest we can read message");
    //     let payload:String= msg.get_payload().unwrap();
    //     let parsed_message: Wmsg =  serde_json::from_str::<Wmsg>(
    //                 &payload.to_string()
    //             ).unwrap();
    //     match (parsed_message.key.as_str()) {
    //                 "relay"=>{
    //                     let mut ws= wsh.lock().unwrap();
    //                     println!("reacged here subs 2");
    //                     ws.broadcast(parsed_message);
                        
    //                 },
    //                 _ => {
    //                     println!("{:?}",parsed_message.key);
    //                 }
                    
    //             }
    //         }
                
    //     });
    //     Ok(())
    // }






}