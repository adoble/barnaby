use std::collections::HashMap;
use chrono::{DateTime, Utc, NaiveDateTime};



#[derive(Debug, Clone)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub datetime: DateTime<Utc>,
}

impl Event {
    /// Creates a new Event instance.
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for the event
    /// * `name` - Name of the event
    /// * `description` - Optional description of the event
    /// * `datetime` - Optional datetime of the event. If None, current time is used
    /// 
    /// # Returns
    /// A new Event instance
    pub fn new(
        id: u32, 
        name: String, 
        description: Option<String>, 
        datetime: Option<DateTime<Utc>>
    ) -> Self {
        Event {
            id,
            name,
            description,
            datetime: datetime.unwrap_or_else(Utc::now),
        }
    }

    /// Updates the event's datetime based on a string input.
    /// Accepts formats like:
    /// - "2024-04-29 14:30:00"
    /// - "2024-04-29T14:30:00Z"
    /// 
    /// # Arguments
    /// * `datetime_str` - A string representing the new datetime
    /// 
    /// # Returns
    /// * `Result<(), String>` - Ok if parsing succeeds, Err with message if it fails
    pub fn update_datetime(&mut self, datetime_str: &str) -> Result<(), String> {
        // Try parsing as RFC3339/ISO 8601
        if let Ok(dt) = DateTime::parse_from_rfc3339(datetime_str) {
            self.datetime = dt.with_timezone(&Utc);
            return Ok(());
        }

        // Try parsing as naive datetime (YYYY-MM-DD HH:MM:SS)
        match NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
            Ok(dt) => {
                self.datetime = DateTime::from_naive_utc_and_offset(dt, Utc);
                Ok(())
            }
            Err(_) => Err("Invalid datetime format. Use YYYY-MM-DD HH:MM:SS or RFC3339".to_string())
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
            "Test Event".to_string(),
            Some("Test Description".to_string()),
            None,
        );

        assert_eq!(event.id, 1);
        assert_eq!(event.name, "Test Event");
        assert_eq!(event.description, Some("Test Description".to_string()));
        
        // Test with specific datetime
        let specific_time = Utc::now();
        let event_with_time = Event::new(
            2,
            "Test Event 2".to_string(),
            None,
            Some(specific_time),
        );
        
        assert_eq!(event_with_time.datetime, specific_time);
    }
}