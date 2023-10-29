
use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use futures::{stream::SplitSink, SinkExt};
use crate::managers::unitsocket::UnitSocket;
use crate::schemas::ws::Wmsg;
use axum::extract::ws::Message;


pub struct FileSocket{
pub file_id: i64,
pub active_sockets:Vec<UnitSocket>,

    
}



impl FileSocket{
    pub fn new(file_id:i64)->FileSocket{
        return FileSocket{
            file_id: file_id,
            active_sockets:Vec::new()
        }
    }

    pub fn add_connection(&mut self,user_id:i64,connection:SplitSink<WebSocket,Message>){
        let id= self.active_sockets.len();
        let mut new_connection= UnitSocket::new(user_id,id.try_into().unwrap(),connection);
        //new_connection.listen().await;
        self._add_connection(new_connection);
        

        println!("unit scoket added to filesocket");
       
    }

    pub fn remove_connection(&mut self,user_id :i64){
        let mut ind=0;
        
        for (index,conn) in self.active_sockets.iter_mut().enumerate(){
            if conn.user_id==user_id{
                ind=index;
            }
        }
    let mut removed_element=self.active_sockets.remove(ind);
    removed_element.close();

    }

   fn  _add_connection(&mut self,  mut connection:UnitSocket){
        let len=self.active_sockets.len();
        println!("add connection file socket");
        self.active_sockets.push(connection);
            
    }

    fn _remove_connection(&mut self,connection:UnitSocket){
        self.active_sockets.remove(connection.id.try_into().unwrap());
    }

    pub async fn broadcast(&mut self, msg:Wmsg){
        println!("Broadcas outside loop");
        for conn in self.active_sockets.iter_mut(){
            println!("Broadcas indie loop");
            let serialized_msg:Message=Message::Text(msg.content.clone());
            conn.connection.send(serialized_msg).await;

        }
    }

    










}