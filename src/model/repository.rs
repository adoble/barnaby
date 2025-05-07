use super::event::{Event, Events};
use super::location::{Location, Locations};
use super::object::{Object, Objects};
use super::person::{Person, Persons};
use super::relationship::{Relationship, Relationships};

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

    /// Add a person to the repository. If a person with the same name exists,
    /// return their id, otherwise add the new person with a new id.
    pub fn add_person(&mut self, mut person: Person) -> u32 {
        if let Some(existing_person) = self.persons.find(&person.name) {
            existing_person.id()
        } else {
            person.id = self.new_id();
            self.persons.add(person)
        }
    }

    // pub fn add_person(&mut self, person: Person) -> u32 {
    //     self.persons.add(person)
    // }

    // TODO change these so that they return the id as an EntityType
    pub fn add_location(&mut self, location: Location) {
        self.locations.add(location);
    }
    pub fn add_event(&mut self, event: Event) {
        self.events.add(event);
    }
    pub fn add_object(&mut self, object: Object) {
        self.objects.add(object);
    }
    pub fn add_relationship(&mut self, relationship: Relationship) {
        self.relationships.add(relationship);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_person() {
        let mut repo = Repository::new();
        let tom = Person::new("Thomas Barnaby".to_string(), Some("DCI".to_string()));

        let id1 = repo.add_person(tom);
        assert_eq!(id1, 1);

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
}
