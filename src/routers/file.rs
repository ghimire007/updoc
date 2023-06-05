use crate::controllers::file::{create_file,update_file,delete_file};
use axum::{routing::get, routing::post,routing::patch,routing::delete ,Router, middleware::{self, Next},};
use crate::middlewares::auth::auth; 

pub fn routes() -> Router {
    Router::new().nest(
        "/file",
        Router::new()
            .route("/create", post(create_file))
            .route("/update/:id", patch(update_file))
            .route("/delete/:id", delete(delete_file))
            .route_layer(middleware::from_fn(auth))
    )
}  