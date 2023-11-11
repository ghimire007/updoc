use crate::controllers::file::{create_file, delete_file, update_file};
use crate::middlewares::auth::auth;
use axum::{
    middleware::{self, Next},
    routing::delete,
    routing::get,
    routing::patch,
    routing::post,
    Router,
};

pub fn routes() -> Router {
    Router::new().nest(
        "/file",
        Router::new()
            .route("/create", post(create_file))
            .route("/update/:id", patch(update_file))
            .route("/delete/:id", delete(delete_file))
            .route_layer(middleware::from_fn(auth)),
    )
}
