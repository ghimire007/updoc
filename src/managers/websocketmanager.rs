use std::collections::HashMap;

use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use futures::stream::SplitSink;

use crate::{managers::filesocket::FileSocket, schemas::ws::Wmsg};



pub struct GlobalWebSocketManager{
    pub file_socket_map:HashMap<String,FileSocket>
    
}


impl GlobalWebSocketManager{

    pub fn new()->GlobalWebSocketManager{
        GlobalWebSocketManager{
            file_socket_map:HashMap::new()
        }
    }

    pub  fn  add_connection(&mut self,file_id: String,user_id:i64,connection:SplitSink<WebSocket,Message>){
        println!("add_connection");

        if let Some(file_socket)=self.file_socket_map.get_mut(&file_id){
            file_socket.add_connection(user_id,connection);
            println!("added to existing file socket group");
          
        }
        else{
            let mut new_file_socket=FileSocket::new(file_id.parse().unwrap());
            new_file_socket.add_connection(user_id,connection);
            self.file_socket_map.insert(file_id,new_file_socket);
            println!("created file socket group");
        }

    }

    pub fn remove_connection(&mut self,file_id: String,user_id:i64)
{
    if let Some(file_socket)=self.file_socket_map.get_mut(&file_id){
        file_socket.remove_connection(user_id);
        println!("sucessfully off boared web scoket connection
        ");
    }

  


}




pub async fn  broadcast(&mut self,msg:Wmsg){
    let file_id=&msg.file_id;
    println!("broad casting to {}",file_id); 
    if let Some(file_socket)=self.file_socket_map.get_mut(file_id)
    {
        println!("broad casting"); 
        file_socket.broadcast(msg).await;// Handle the Some case with `file_socket`
    } else {
        println!("couldnot find  file id")
    }
  

}








}

