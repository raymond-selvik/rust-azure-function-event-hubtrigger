use std::env;

use axum::{
    routing::post,
    Router,
};

pub mod handlers;
pub mod models;
pub mod event_processor;

#[tokio::main]
async fn main() {

    // build our application with a single route
    let app = Router::new()
        .route("/EventHubTrigger", post(handlers::handle_eventhub_event));

    let port = env::var("FUNCTIONS_CUSTOMHANDLER_PORT").unwrap();
    //let port = "8080";

    let host = format!("0.0.0.0:{}", port);
    print!("PORT: {}", port);
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}