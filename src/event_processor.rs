use crate::models::EventHubEvent;

pub fn process_event(event: EventHubEvent) {
    println!("{:?}", event);
    
}