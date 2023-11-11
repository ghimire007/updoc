use crate::managers::websocketmanager::GlobalWebSocketManager;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct AppState {
    pub sManager: Arc<Mutex<GlobalWebSocketManager>>,
    pub publisher: Arc<Mutex<redis::Connection>>,
}
