use std::collections::HashMap;
use super::time::Time;

#[derive(Debug, Clone)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub time: Time,
}

impl Event {
    pub fn new(
        id: u32, 
        name: String, 
        description: Option<String>, 
        time: Option<Time>
    ) -> Self {
        Event {
            id,
            name,
            description,
            time: time.unwrap_or_else(|| Time::new("now")),
        }
    }
}

#[derive(Debug, Default)]
pub struct Events {
    items: HashMap<u32, Event>,
    next_id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_event() {
        let event = Event::new(
            1,
            "Murder at Badger's Drift".to_string(),
            Some("Body found in the woods".to_string()),
            Some(Time("yesterday morning".to_string())),
        );

        assert_eq!(event.id, 1);
        assert_eq!(event.name, "Murder at Badger's Drift");
        assert_eq!(event.time.0, "yesterday morning");

        // Using the new method
        let event2 = Event::new(
            2,
            "Suspicious Activity".to_string(),
            None,
            Some(Time::new("late last night")),
        );
        
        assert_eq!(event2.time.0, "late last night");
    }
}