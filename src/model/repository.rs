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

    pub fn add_person(&mut self, person: Person) {
        self.persons.add(person);
    }
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
