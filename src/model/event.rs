use super::time::Time;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Event {
    pub(super) id: u32,
    pub name: String,
    pub description: Option<String>,
    pub time: Option<Time>,
}

impl Event {
    // Private constructor for Event. This is to restrict the creation of
    // ids to the Repository.
    fn new_with_id(id: u32, name: String, description: Option<String>, time: Option<Time>) -> Self {
        Event {
            id,
            name,
            description,
            time,
        }
    }

    /// Creates a new Event instance.
    ///             
    /// # Arguments
    /// * `name` - Name of the event
    /// * `description` - Optional description of the event
    /// * `time` - Optional time of the event
    ///
    /// # Returns
    /// A new Event instance
    pub fn new(name: String, description: Option<String>, time: Option<Time>) -> Self {
        Event {
            id: 0,
            name,
            description,
            time,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    /// Sets the time of the event
    pub fn set_time(&mut self, time: Time) {
        self.time = Some(time);
    }

    /// Get the time of the event
    pub fn time(&self) -> Option<&Time> {
        self.time.as_ref()
    }

    /// Get the time of the event as &str.
    /// Returns empty string if no time is set.
    pub fn time_as_str(&self) -> &str {
        match &self.time {
            Some(time) => &time.0,
            None => "",
        }
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
    pub fn add(&mut self, event: Event) -> u32 {
        let id = event.id;
        self.0.insert(event.id, event);
        id
    }

    /// Returns the number of events in the collection
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the collection contains no events
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Finds an event by name
    pub fn find(&self, name: &str) -> Option<&Event> {
        self.0.values().find(|event| event.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_event() {
        let event = Event::new_with_id(
            1,
            "Murder at Badger's Drift".to_string(),
            Some("Body found in the woods".to_string()),
            Some(Time("yesterday morning".to_string())),
        );

        assert_eq!(event.id, 1);
        assert_eq!(event.name, "Murder at Badger's Drift");
        assert_eq!(event.time.unwrap().0, "yesterday morning");

        // Test event without time
        let event2 = Event::new_with_id(2, "Suspicious Activity".to_string(), None, None);

        assert!(event2.time.is_none());
        assert_eq!(event2.time_as_str(), "");
    }

    #[test]
    fn test_events_collection() {
        let mut events = Events::new();

        let murder = Event::new_with_id(
            1,
            "Murder at Badger's Drift".to_string(),
            Some("Body found in the woods".to_string()),
            Some(Time("yesterday morning".to_string())),
        );

        events.add(murder);
        assert_eq!(events.0.len(), 1);

        let retrieved = events.0.get(&1).unwrap();
        assert_eq!(retrieved.name, "Murder at Badger's Drift");
        assert_eq!(retrieved.time.as_ref().unwrap().0, "yesterday morning");
    }

    #[test]
    fn test_events_len_and_find() {
        let mut events = Events::new();
        assert_eq!(events.len(), 0);
        assert!(events.is_empty());

        let murder = Event::new_with_id(
            1,
            "Murder at Badger's Drift".to_string(),
            Some("Body found in the woods".to_string()),
            Some(Time("yesterday morning".to_string())),
        );
        events.add(murder);

        assert_eq!(events.len(), 1);
        assert!(!events.is_empty());

        // Test find functionality
        let found = events.find("Murder at Badger's Drift");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, 1);

        // Test non-existent event
        assert!(events.find("Garden Party").is_none());
    }
}
