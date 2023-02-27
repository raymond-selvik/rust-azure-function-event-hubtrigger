use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TriggerResponse {
    pub name: String
}

#[derive(Serialize, PartialEq, Deserialize, Debug, Clone)]
pub struct EventhubRequest {
    #[serde(alias = "Data")]
    pub data: EventhubMessage
}
#[derive(Serialize,PartialEq, Deserialize, Debug, Clone)]
pub struct EventhubMessage {
    #[serde(alias = "eventHubMessages")]
    pub event_hub_message: String
}

#[derive(Debug, Deserialize)]
pub struct EventHubEvent {
    pub timestamp: DateTime<Utc>, 
    pub value: i32,
    pub value2: i32
}

