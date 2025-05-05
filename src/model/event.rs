use super::time::Time;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub time: Time,
}

impl Event {
    /// Creates a new Event instance.
    ///             
    /// # Arguments
    /// * `id` - Unique identifier for the event
    /// * `name` - Name of the event
    /// * `description` - Optional description of the event
    /// * `time` - Optional time of the event
    ///
    /// # Returns
    /// A new Event instance
    pub fn new(id: u32, name: String, description: Option<String>, time: Option<Time>) -> Self {
        Event {
            id,
            name,
            description,
            time: time.unwrap_or_else(|| Time::new("now")),
        }
    }

    /// Sets the time of the event
    pub fn set_time(&mut self, time: Time) {
        self.time = time;
    }
}

#[derive(Debug, Default)]
pub struct Events(pub HashMap<u32, Event>);

impl Events {
    /// Creates a new empty Events collection
    pub fn new() -> Self {
        Events(HashMap::new())
    }

    /// Adds an event to the collection
    pub fn add(&mut self, event: Event) {
        self.0.insert(event.id, event);
    }
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

    #[test]
    fn test_events_collection() {
        let mut events = Events::new();

        let murder = Event::new(
            1,
            "Murder at Badger's Drift".to_string(),
            Some("Body found in the woods".to_string()),
            Some(Time("yesterday morning".to_string())),
        );

        events.add(murder);
        assert_eq!(events.0.len(), 1);

        let retrieved = events.0.get(&1).unwrap();
        assert_eq!(retrieved.name, "Murder at Badger's Drift");
        assert_eq!(retrieved.time.0, "yesterday morning");
    }
}
