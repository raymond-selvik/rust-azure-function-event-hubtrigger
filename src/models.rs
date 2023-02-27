use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Responsemsg {
    pub name: String
}

#[derive(Serialize, PartialEq, Deserialize, Debug, Clone)]
pub struct EventhubRequest {
    #[serde(alias = "Data")]
    pub data: EventhubMessage
}
#[derive(Serialize,PartialEq, Deserialize, Debug, Clone)]
pub struct EventhubMessage {
    pub eventHubMessages: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventHubEvent {
    pub timestamp: String, 
    pub value: i32,
    pub value2: i32
}
