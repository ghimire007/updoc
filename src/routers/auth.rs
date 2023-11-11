use crate::controllers::auth::{login_user, signup_user, test_user};
use crate::controllers::ws::handler;
use crate::middlewares::auth::auth;
use axum::{
    middleware::{self, Next},
    routing::get,
    routing::post,
    Router,
};

pub fn routes() -> Router {
    Router::new().nest(
        "/auth",
        Router::new()
            .route("/test", post(test_user))
            .route_layer(middleware::from_fn(auth))
            .route("/signup", post(signup_user))
            .route("/login", post(login_user)), // .route("/ws/file/:id",get(handler)),
    )
}
