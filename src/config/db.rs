use crate::config::config::get_env;
use lazy_static::lazy_static;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

// #[tokio::main]
// pub async fn get_db()->DatabaseConnection{
// let mut opt = ConnectOptions::new(get_env("DATABASE_URL").to_owned());
// opt.max_connections(10)
//     .min_connections(5)
//     .connect_timeout(Duration::from_secs(8))
//     .acquire_timeout(Duration::from_secs(8))
//     .idle_timeout(Duration::from_secs(8))
//     .max_lifetime(Duration::from_secs(8))
//     .sqlx_logging(true);
//     //.sqlx_logging_level(log::LevelFilter::Info)
//     //.set_schema_search_path("my_schema".into()); // Setting default PostgreSQL schema

// let db = Database::connect(opt).await.unwrap();
// return db;

// }
// lazy_static! {
//     pub static ref DB: DatabaseConnection = get_db();
// }

lazy_static! {
    #[derive(Debug)]
    pub static ref DB: DatabaseConnection = {
        //get_db();
        let mut opt = ConnectOptions::new(get_env("DATABASE_URL").to_owned());
        opt.max_connections(10)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);
            //.sqlx_logging_level(log::LevelFilter::Info)
            //.set_schema_search_path("my_schema".into()); // Setting default PostgreSQL schema

        // let db = tokio::runtime::Runtime::new()
        //     .unwrap()
        //     .block_on(async move { Database::connect(opt).await.unwrap() });

        let db=  tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async move {
                Database::connect(opt).await.unwrap()
            })
        });

        println!("db connected");



        db
    };
}
