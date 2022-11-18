use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use common::service::database;

mod error;
mod handler;

#[tokio::main]
async fn main() {
    let database_pool = database::create_pool().await;

    let app = Router::new()
        .route("/status", get(handler::status::handler))
        .route("/accounts", post(handler::create_account::handler))
        .layer(Extension(database_pool));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
