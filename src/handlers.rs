
use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{models, event_processor};

pub async fn handle_eventhub_event(Json(body): Json<models::EventhubRequest>) -> impl IntoResponse {
    println!("{}", body.data.event_hub_message);

    let event = parse_event_from_string(&body.data.event_hub_message);

    event_processor::process_event(event);

    let response = models::TriggerResponse {
        name: String::from("Hello")
    };
    
    (StatusCode::OK, Json(response))
}

fn parse_event_from_string(event: &str) -> models::EventHubEvent {
    let raw = event.replace("\\", "");
    let raw = &raw[1..raw.len() - 1];

    let event = serde_json::from_str::<models::EventHubEvent>(&raw).unwrap();

    return event;
}
