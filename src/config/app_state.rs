use crate::managers::websocketmanager::GlobalWebSocketManager;
use std::{net::SocketAddr, sync::{Mutex,Arc}};



#[derive(Clone)]
pub struct AppState{
    pub  sManager:Arc<Mutex<GlobalWebSocketManager>>,
    pub  publisher:Arc<Mutex<redis::Connection>>
    

}