#![allow(dead_code)]

// use std::fs;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::model::{location::Location, person::Person, repository::Repository};

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
                Rule::alias => println!("Alias"),
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
            Rule::person => process_person(pair, repository),
            // Rule::event => println!("Event: {}", pair.as_str()),
            Rule::location => process_location(pair, repository),
            Rule::object => process_object(pair, repository),
            // Rule::alias => println!("Alias"),
            // Rule::entity => process_entity(pair),
            // Rule::relationship => println!("Relationship"),
            _ => (),
        }
    }
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

pub fn process_person(pair: Pair<'_, Rule>, repository: &mut Repository) {
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
    let person_id = repository.new_id();
    let mut person = Person::new(person_id, name.to_string(), notes);

    match person_qualifier {
        PersonQualifier::None => (),
        PersonQualifier::Victim => person.set_victim_status(true),
        PersonQualifier::Witness => todo!(),
        PersonQualifier::Suspect => todo!(),
        PersonQualifier::Other(_) => todo!(),
    }

    repository.add_person(person);
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
}
