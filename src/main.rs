use std::env;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use unescape::unescape;



#[tokio::main]
async fn main() {

    // build our application with a single route
    let app = Router::new()
        .route("/EventHubTrigger", post(post_event));

    let port = env::var("FUNCTIONS_CUSTOMHANDLER_PORT").unwrap();
    //let port = "8080";

    let host = format!("0.0.0.0:{}", port);
    print!("PORT: {}", port);
    axum::Server::bind(&host.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}



async fn post_event(Json(body): Json<EventhubRequest>) -> impl IntoResponse {
    println!("{}", body.data.eventHubMessages);
    let raw = body.data.eventHubMessages.as_str();
    println!("{}", raw);

    let raw = raw.replace("\\", "");
    let raw = &raw[1..raw.len() - 1];
    //let raw = unescape(raw).unwrap();
    //println!("{}", raw);
    let event = serde_json::from_str::<EventHubEvent>(raw).unwrap();
    println!(" BODY {}", event.timestamp);
    println!(" BODY {}", event.value);  
    println!(" BODY {}", event.value2);


    let response = Responsemsg {
        name: String::from("Hello")
    };
    
    (StatusCode::OK, Json(response))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Responsemsg {
    name: String
}

#[derive(Serialize, PartialEq, Deserialize, Debug, Clone)]
struct EventhubRequest {
    #[serde(alias = "Data")]
    data: EventhubMessage
}
#[derive(Serialize,PartialEq, Deserialize, Debug, Clone)]
struct EventhubMessage {
    eventHubMessages: String
}

#[derive(Debug, Serialize, Deserialize)]
struct EventHubEvent {
    timestamp: String, 
    value: i32,
    value2: i32
}