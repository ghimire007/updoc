#![allow(warnings)]
//use axum::{routing::get, Router};
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    extract::State,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use redis::Commands;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    thread,
};

mod config;
mod controllers;
mod crdt;
mod error;
mod managers;
mod middlewares;
mod models;
mod routers;
mod schemas;

use crate::controllers::ws::handler;
use config::app_state::AppState;
use config::config::get_env;
use config::redis::{connect, run_subscriber};
use managers::websocketmanager::GlobalWebSocketManager;
use migration::{Migrator, MigratorTrait, SeaRc};
use routers::auth;
use routers::file;

#[tokio::main]

async fn main() {
    let connection = sea_orm::Database::connect(&get_env("DATABASE_URL"))
        .await
        .unwrap();
    println!("{:?}", connection);
    //Migrator::down(&connection, None).await.unwrap();

    let st = AppState {
        // sManager: Arc::new(Mutex::new(GlobalWebSocketManager::new()))
        sManager: Arc::new(Mutex::new(GlobalWebSocketManager::new())),
        publisher: Arc::new(Mutex::new(connect())),
    };
    run_subscriber();

    let route = Router::new()
        .route("/ws/:id", get(handler))
        .with_state(st)
        .nest("/api/v1", auth::routes().merge(file::routes()));

    let address = SocketAddr::from(([127, 0, 0, 1], 7777));
    axum::Server::bind(&address)
        .serve(route.into_make_service())
        .await
        .unwrap();
}
