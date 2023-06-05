#![allow(warnings)]
use axum::{routing::get, Router};
use std::net::SocketAddr;

mod config;
mod controllers;
mod error;
mod models;
mod routers;
mod schemas;
mod middlewares;
use routers::auth;
use routers::file;
use config::config::get_env;
//use config::db::DB;

use migration::{Migrator, MigratorTrait};

// let router1 = Router::new()
//     .route("/users", get(|| async { "List of users" }))
//     .route("/posts", get(|| async { "List of posts" }));

// let router2 = Router::new()
//     .nest("/api/v1", router1);

// let router3 = Router::new()
//     .nest("/api", router2)
//     .route("/", get(|| async { "Hello, world!" }));

// let app = Router::new().merge(router3);

#[tokio::main]
async fn main() {
    // let connection_result = sea_orm::Database::connect(get_env("DATABASE_URL").to_string()).await;
    // match connection_result {
    //     Ok(connection) => {
    //         println!("connecting to database: {:?}", connection);
    //     }
    //     Err(error) => {
    //         eprintln!("Error connecting to database: {}", error);
    //         // Handle the error in some way
    //     }
    // }

    let connection = sea_orm::Database::connect(&get_env("DATABASE_URL"))
        .await
        .unwrap();
    println!("{:?}", connection);
    //Migrator::down(&connection, None).await.unwrap();

    let route = Router::new().nest("/api/v1", 
    auth::routes().merge(file::routes()),

);
    let address = SocketAddr::from(([127, 0, 0, 1], 7777));
    axum::Server::bind(&address)
        .serve(route.into_make_service())
        .await
        .unwrap();
}
