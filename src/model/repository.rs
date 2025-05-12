use super::event::{Event, Events};
use super::location::{Location, Locations};
use super::object::{Object, Objects};
use super::person::{Person, Persons};
use super::relationship::{Relationship, Relationships};
// use super::time::Time;
use super::EntityType;

pub struct Repository {
    pub relationships: Relationships,
    pub persons: Persons,
    pub locations: Locations,
    pub events: Events,
    pub objects: Objects,
    pub highest_id: u32,
}

impl Repository {
    pub fn new() -> Repository {
        Repository {
            relationships: Relationships::new(),
            persons: Persons::new(),
            locations: Locations::new(),
            events: Events::new(),
            objects: Objects::new(),
            highest_id: 0,
        }
    }

    pub fn new_id(&mut self) -> u32 {
        self.highest_id += 1;
        self.highest_id
    }

    // TODO  the following can be made more generic ...

    /// Add a person to the repository. If a person with the same name exists,
    /// return their id, otherwise add the new person with a new id.
    pub fn add_person(&mut self, mut person: Person) -> EntityType {
        let raw_id = if let Some(existing_person) = self.persons.find(&person.name) {
            existing_person.id()
        } else {
            person.id = self.new_id();
            self.persons.add(person)
        };

        EntityType::Person(raw_id)
    }

    pub fn add_location(&mut self, mut location: Location) -> EntityType {
        let raw_id = if let Some(existing_location) = self.locations.find(&location.name) {
            existing_location.id()
        } else {
            location.id = self.new_id();
            self.locations.add(location)
        };
        EntityType::Location(raw_id)
    }

    pub fn add_event(&mut self, mut event: Event) -> EntityType {
        let raw_id = if let Some(existing_event) = self.events.find(&event.name) {
            existing_event.id()
        } else {
            event.id = self.new_id();
            self.events.add(event)
        };

        EntityType::Event(raw_id)
    }

    pub fn add_object(&mut self, mut object: Object) -> EntityType {
        let raw_id = if let Some(existing_object) = self.objects.find(&object.name) {
            existing_object.id()
        } else {
            object.id = self.new_id();
            self.objects.add(object)
        };

        EntityType::Object(raw_id)
    }

    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.add(relationship);
    }

    /// Returns a formatted string representing the current state of the repository
    pub fn display_state(&self) -> String {
        let mut output = String::new();

        output.push_str("REPOSITORY --> ");

        // Display Persons
        output.push_str("PERSONS:\n");
        for person in self.persons.iter() {
            output.push_str(&format!("  [{}] {}", person.id(), person.name));
            if let Some(desc) = &person.description {
                output.push_str(&format!(" ({})", desc));
            }
            if !person.aliases.is_empty() {
                output.push_str(&format!(" aka: {:?}", person.aliases));
            }
            output.push('\n');
        }

        // Display Locations
        output.push_str("\nLOCATIONS:\n");
        for location in self.locations.iter() {
            output.push_str(&format!("  [{}] {}", location.id(), location.name));
            if let Some(desc) = &location.description {
                output.push_str(&format!(" ({})", desc));
            }
            output.push('\n');
        }

        // Display Objects
        output.push_str("\nOBJECTS:\n");
        for object in self.objects.iter() {
            output.push_str(&format!("  [{}] {}", object.id(), object.name));
            if let Some(desc) = &object.description {
                output.push_str(&format!(" ({})", desc));
            }
            output.push('\n');
        }

        // Display Events
        output.push_str("\nEVENTS:\n");
        for event in self.events.iter() {
            output.push_str(&format!("  [{}] {}", event.id(), event.name));
            if let Some(desc) = &event.description {
                output.push_str(&format!(" ({})", desc));
            }
            if let Some(time) = &event.time {
                output.push_str(&format!(" at {}", time.0));
            }
            output.push('\n');
        }

        // Display Relationships
        output.push_str("\nRELATIONSHIPS:\n");
        for rel in self.relationships.iter() {
            output.push_str(&format!(
                "  {} → {} : {}",
                self.entity_to_string(&rel.from),
                self.entity_to_string(&rel.to),
                rel.relationship_type
            ));
            if !rel.time.is_empty() {
                output.push_str(&format!(
                    " ({})",
                    rel.time
                        .iter()
                        .map(|t| t.0.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            if let Some(notes) = &rel.notes {
                output.push_str(&format!(" [{}]", notes));
            }
            output.push('\n');
        }

        output
    }

    /// TODO use the Display trait for this
    /// Helper function to convert EntityType to string representation
    fn entity_to_string(&self, entity: &EntityType) -> String {
        match entity {
            EntityType::Person(id) => {
                if let Some(person) = self.persons.get(*id) {
                    format!("Person({}:{})", id, person.name)
                } else {
                    format!("Person({})", id)
                }
            }
            EntityType::Location(id) => {
                if let Some(location) = self.locations.get(*id) {
                    format!("Location({}:{})", id, location.name)
                } else {
                    format!("Location({})", id)
                }
            }
            EntityType::Event(id) => {
                if let Some(event) = self.events.get(*id) {
                    format!("Event({}:{})", id, event.name)
                } else {
                    format!("Event({})", id)
                }
            }
            EntityType::Object(id) => {
                if let Some(object) = self.objects.get(*id) {
                    format!("Object({}:{})", id, object.name)
                } else {
                    format!("Object({})", id)
                }
            }
            EntityType::Unknown => "Unknown".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Time;

    use super::*;

    #[test]
    fn test_add_person() {
        let mut repo = Repository::new();
        let tom = Person::new("Thomas Barnaby".to_string(), Some("DCI".to_string()));

        let id1 = repo.add_person(tom);
        assert_eq!(id1, EntityType::Person(1));

        // Try to add same person again
        let tom_again = Person::new(
            "Thomas Barnaby".to_string(),  // Same name
            Some("Detective".to_string()), // Different description
        );
        let id2 = repo.add_person(tom_again);
        assert_eq!(id1, id2); // Should return same ID as first addition

        // Add different person
        let joyce = Person::new("Joyce Barnaby".to_string(), None);
        let id3 = repo.add_person(joyce);
        assert_ne!(id3, id1);

        // Verify persons were stored correctly
        assert!(repo.persons.find("Thomas Barnaby").is_some());
        assert!(repo.persons.find("Joyce Barnaby").is_some());
        assert_eq!(repo.persons.len(), 2); // Only two people should be stored
    }

    #[test]
    fn test_add_location() {
        let mut repo = Repository::new();

        let vicarage = Location::new("St. Michael's Vicarage".to_string(), None);
        let id1 = repo.add_location(vicarage);
        assert_eq!(id1, EntityType::Location(1));

        // Try to add same location again
        let vicarage_again = Location::new(
            "St. Michael's Vicarage".to_string(),
            Some("Where the murder weapon was found".to_string()),
        );
        let id2 = repo.add_location(vicarage_again);
        assert_eq!(id1, id2); // Should return same ID

        assert_eq!(repo.locations.len(), 1); // Only one location stored
    }

    #[test]
    fn test_display_state() {
        let mut repo = Repository::new();

        // Add test data
        let tom = Person::new("Thomas Barnaby".to_string(), Some("DCI".to_string()));
        let id1 = repo.add_person(tom);

        let vicarage = Location::new("St. Michael's Vicarage".to_string(), None);
        let id2 = repo.add_location(vicarage);

        let sword = Object::new(
            "Indian Sword".to_string(),
            Some("Decorative weapon".to_string()),
        );
        let _id3 = repo.add_object(sword);

        repo.add_relationship(Relationship {
            from: id1,
            to: id2,
            relationship_type: "investigating at".to_string(),
            time: vec![Time("morning".to_string())],
            notes: Some("Found murder weapon".to_string()),
        });

        let state = repo.display_state();

        // Basic verification
        assert!(state.contains("Thomas Barnaby"));
        assert!(state.contains("St. Michael's Vicarage"));
        assert!(state.contains("Indian Sword"));
        assert!(state.contains("investigating at"));
        assert!(state.contains("morning"));
        assert!(state.contains("Found murder weapon"));
    }
}
