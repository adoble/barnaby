#![allow(dead_code)]

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::model::{
    event::Event, location::Location, person::Person, repository::Repository, time::Time,
};

use super::object_qualifier::ObjectQualifier;
use super::person_qualifier::PersonQualifier;

#[derive(Parser)]
#[grammar = "troy.pest"]
pub struct TroyParser;

impl TroyParser {
    pub fn build_model(statement: &str, repository: &mut Repository) {
        let mut parse = TroyParser::parse(Rule::statement, statement).expect("unsuccessful parse");

        let pair: Pair<'_, Rule> = parse.next().unwrap();

        let inner_rules = pair.into_inner();

        for pair in inner_rules {
            match pair.as_rule() {
                Rule::person => println!("Person: {}", pair.as_str()),
                Rule::event => println!("Event: {}", pair.as_str()),
                Rule::location => println!("Location: {}", pair.as_str()),
                Rule::object => println!("Object: {}", pair.as_str()),
                Rule::alias => process_alias(pair, repository),
                Rule::entity => process_entity(pair, repository),
                Rule::relationship => println!("Relationship"),
                _ => (),
            }
        }
    }
}

pub fn process_entity(pair: Pair<'_, Rule>, repository: &mut Repository) {
    let inner_rules = pair.into_inner();

    for pair in inner_rules {
        match pair.as_rule() {
            Rule::person => {
                // TODO  alle process_* function should return EntityType(id)
                let _ = process_person(pair, repository);
            }
            Rule::event => process_event(pair, repository),
            Rule::location => process_location(pair, repository),
            Rule::object => process_object(pair, repository),
            // Rule::alias => println!("Alias"),
            // Rule::entity => process_entity(pair),
            // Rule::relationship => println!("Relationship"),
            _ => (), // TODO change the type of an id so none is possible
        };
    }
}

// Process and event
pub fn process_event(pair: Pair<'_, Rule>, repository: &mut Repository) {
    let mut name = "";

    let mut time: Option<Time> = None;
    // TODO
    let description = None;

    let inner_rules = pair.into_inner();
    for pair in inner_rules {
        match pair.as_rule() {
            Rule::id => name = pair.as_str(),
            Rule::time => time = process_time(pair),

            _ => println!("Other: {}", pair.as_str()),
        }
    }
    let event_id = repository.new_id();

    let event = Event::new(event_id, name.to_string(), description, time);

    repository.add_event(event);
}
// Process an alias
pub fn process_alias(pair: Pair<'_, Rule>, repository: &mut Repository) {
    println!("DEBUG: process_alias: {}", pair.as_str());

    let mut alias = None;
    // let mut found_person: Option<&mut Person> = None;
    let mut found_person: Option<&mut Person> = None;
    let found_person_id: Option<u32> = None;

    let inner_rules = pair.into_inner();
    for pair in inner_rules {
        match pair.as_rule() {
            Rule::person => {
                // found_person = repository.persons.find_mut(pair.as_str());
                found_person = repository.persons.find_mut(pair.as_str());

                if found_person.is_some() {
                } else {
                    // Add a new person
                    process_person(pair, repository);
                }
            }
            Rule::id => alias = Some(pair.as_str().to_string()),
            _ => (),
        }
    }

    //repository.persons.get_mut(id)

    // if found_person.is_some() {
    //     let person = found_person.unwrap();
    //     person.add_alias(alias.unwrap());
    // } else {
    //     println!("ERROR: Person not found"); // TODO proper error handling
    // }
}

// Process a time
pub fn process_time(pair: Pair<'_, Rule>) -> Option<Time> {
    let mut time: Option<Time> = None;

    let inner_rules = pair.into_inner();
    for pair in inner_rules {
        match pair.as_rule() {
            Rule::id => {
                time = Some(Time::new(pair.as_str()));
            }
            _ => (),
        }
    }
    time
}

// Process a location
pub fn process_location(pair: Pair<'_, Rule>, repository: &mut Repository) {
    let mut name = "";

    // TODO
    let description = None;

    let inner_rules = pair.into_inner();
    for pair in inner_rules {
        match pair.as_rule() {
            Rule::id => name = pair.as_str(),
            _ => (),
        }
    }
    let location_id = repository.new_id();
    let location = Location::new(location_id, name.to_string(), description);

    repository.add_location(location);
}

// Process an object
pub fn process_object(pair: Pair<'_, Rule>, repository: &mut Repository) {
    let mut name = "";
    let mut object_qualifier = ObjectQualifier::None;

    // TODO
    let description = None;

    let inner_rules = pair.into_inner();
    for pair in inner_rules {
        match pair.as_rule() {
            Rule::id => name = pair.as_str(),
            Rule::object_qualifier => object_qualifier = ObjectQualifier::from_str(pair.as_str()),
            _ => (),
        }
    }
    let object_id = repository.new_id();
    let mut object = crate::model::object::Object::new(object_id, name.to_string(), description);

    match object_qualifier {
        ObjectQualifier::None => (),
        ObjectQualifier::Weapon => object.set_type(crate::model::object::ObjectType::Weapon),
        _ => todo!(),
    }

    // TODO ObjectType and ObjectQualifier seem to be redundant

    repository.add_object(object);
}

pub fn process_person(pair: Pair<'_, Rule>, repository: &mut Repository) -> u32 {
    let mut name = "";
    let mut person_qualifier = PersonQualifier::None;

    //TODO
    let notes: Option<String> = None;

    let inner_rules = pair.into_inner();
    for pair in inner_rules {
        match pair.as_rule() {
            Rule::id => name = pair.as_str(),
            Rule::person_qualifier => person_qualifier = PersonQualifier::from_str(pair.as_str()),
            _ => (),
        }
    }
    //let person_id = repository.new_id();
    let mut person = Person::new(name.to_string(), notes);

    // TODO add other qualifiers to Person in the grammer
    match person_qualifier {
        PersonQualifier::None => (),
        PersonQualifier::Victim => person.set_victim_status(true),
        PersonQualifier::Witness => todo!(),
        PersonQualifier::Suspect => todo!(),
        PersonQualifier::Other(_) => todo!(),
    }

    repository.add_person(person)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_person() {
        let mut repo = &mut Repository::new();
        TroyParser::build_model("pv Richard Bayly", &mut repo);

        assert_eq!(repo.persons.len(), 1); // Ensure the length method is implemented

        let person = repo.persons.find("Richard Bayly");
        assert!(person.is_some());
        // TODO finding more then one person?

        let person = person.unwrap();
        assert_eq!(person.name, "Richard Bayly");
        assert!(person.is_victim);
    }

    #[test]
    fn test_parse_object() {
        let mut repo = &mut Repository::new();
        TroyParser::build_model("o Red Dress", &mut repo);

        assert_eq!(repo.objects.len(), 1);

        let object = repo.objects.find("Red Dress");
        assert!(object.is_some());

        let object = object.unwrap();
        assert_eq!(object.name, "Red Dress");

        // weapon qualifier
        TroyParser::build_model("ow Indian Knife", &mut repo);

        assert_eq!(repo.objects.len(), 2);

        let object = repo.objects.find("Indian Knife");
        assert!(object.is_some());

        let object = object.unwrap();
        assert_eq!(object.name, "Indian Knife");
        assert!(object.is_weapon());
    }

    #[test]
    fn test_parse_location() {
        let mut repo = &mut Repository::new();
        TroyParser::build_model("l Old Quarry", &mut repo);

        assert_eq!(repo.locations.len(), 1);

        let location = repo.locations.find("Old Quarry");
        assert!(location.is_some());

        let location = location.unwrap();
        assert_eq!(location.name, "Old Quarry");
    }

    #[test]
    fn test_parse_event() {
        let mut repo = Repository::new();
        TroyParser::build_model("e Robbery", &mut repo);

        assert_eq!(repo.events.len(), 1);

        let event = repo.events.find("Robbery");
        assert!(event.is_some());

        let event = event.unwrap();
        assert_eq!(event.name, "Robbery");
    }

    #[test]
    fn test_parse_event_with_time() {
        let mut repo = Repository::new();
        TroyParser::build_model("e Robbery  t yesterday", &mut repo);

        assert_eq!(repo.events.len(), 1);

        let event = repo.events.find("Robbery");
        assert!(event.is_some());

        let event = event.unwrap();
        assert_eq!(event.name, "Robbery");

        assert_eq!(event.time_as_str(), "yesterday");
        // TODO check time
    }

    #[test]
    fn test_parse_aka() {
        let mut repo = Repository::new();
        TroyParser::build_model("p Robert Bayly  aka  Bob", &mut repo);

        assert_eq!(repo.persons.len(), 1);

        let person = repo.persons.find("Robert Bayly");
        assert!(person.is_some());

        let person = person.unwrap();
        assert_eq!(person.name, "Robert Bayly");

        assert_eq!(person.number_aliases(), 1);

        let aliases: Vec<String> = person.aliases().map(|s| s.to_string()).collect();
        assert_eq!(aliases, vec!["Bob"]);
    }
}
