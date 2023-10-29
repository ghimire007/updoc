#![allow(warnings)]
//use axum::{routing::get, Router};
use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    extract::State,
    routing::get,
    response::{IntoResponse, Response},
    Router,
};
use redis::{Commands,};
use std::{net::SocketAddr, sync::{Mutex,Arc}, thread};

mod config;
mod error;
mod models;
mod routers;
mod schemas;
mod middlewares;
mod managers;
mod controllers;


use routers::auth;
use routers::file;
use config::config::get_env;
use config::app_state::AppState;
use managers::websocketmanager::GlobalWebSocketManager;
use crate::controllers::ws::{handler};
use migration::{Migrator, MigratorTrait, SeaRc};
use config::redis::{run_subscriber,connect};



#[tokio::main]

async fn main() {

    let connection = sea_orm::Database::connect(&get_env("DATABASE_URL"))
        .await
        .unwrap();
    println!("{:?}", connection);
    //Migrator::down(&connection, None).await.unwrap();

   
    let st=AppState{
        // sManager: Arc::new(Mutex::new(GlobalWebSocketManager::new()))
        sManager: Arc::new(Mutex::new(GlobalWebSocketManager::new())),
        publisher: Arc::new(Mutex::new(connect()))

        
    };
    run_subscriber();

    let route = Router::new()
    .route("/ws/:id", get(handler)).with_state(st)
    .nest("/api/v1", auth::routes().merge(file::routes()));
    
    

    let address = SocketAddr::from(([127, 0, 0, 1], 7777));
    axum::Server::bind(&address)
        .serve(route.into_make_service())
        .await
        .unwrap();
}
