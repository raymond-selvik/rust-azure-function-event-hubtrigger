
use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{models, event_processor};

pub async fn handle_eventhub_event(Json(body): Json<models::EventhubRequest>) -> impl IntoResponse {
    println!("{}", body.data.eventHubMessages);
    let raw = body.data.eventHubMessages;
    println!("{}", raw);

    let raw = raw.replace("\\", "");
    let raw = &raw[1..raw.len() - 1];
    //let raw = unescape(raw).unwrap();
    //println!("{}", raw);
    let event = serde_json::from_str::<models::EventHubEvent>(&raw).unwrap();
    println!(" BODY {}", event.timestamp);
    println!(" BODY {}", event.value);  
    println!(" BODY {}", event.value2);

    event_processor::process_event(event);
 

    let response = models::Responsemsg {
        name: String::from("Hello")
    };
    
    (StatusCode::OK, Json(response))
}


