#![allow(dead_code)]

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::model::relationship::Relationship;
use crate::model::{EntityType, Event, Location, Object, Person, Repository, Time};

use super::object_qualifier::ObjectQualifier;
use super::person_qualifier::PersonQualifier;

#[derive(Parser)]
#[grammar = "troy.pest"]
pub struct TroyParser;

impl TroyParser {
    pub fn build_model(
        statement: &str,
        repository: &mut Repository,
    ) -> Result<(), pest::error::Error<Rule>> {
        println!("DEBUG: statement=  {}", statement);

        let mut source_entity_id = EntityType::Unknown;

        let mut parse = TroyParser::parse(Rule::statement, statement)?;

        let pair: Pair<'_, Rule> = parse.next().unwrap();

        let inner_rules = pair.into_inner();

        for pair in inner_rules {
            match pair.as_rule() {
                Rule::relation => process_relationship(source_entity_id, pair, repository),
                Rule::entity => source_entity_id = process_entity(pair, repository),
                _ => (),
            }
        }

        Ok(())
    }
}

pub fn process_relationship(
    source_entity_id: EntityType,
    pair: Pair<'_, Rule>,
    repository: &mut Repository,
) {
    assert!(source_entity_id != EntityType::Unknown);

    let mut target_entity_id = EntityType::Unknown;
    let mut link_id = "".to_string();
    let mut times = Vec::new();

    let inner_rules = pair.into_inner();

    for pair in inner_rules {
        match pair.as_rule() {
            Rule::link => (link_id, times) = process_link(pair),
            Rule::entity => target_entity_id = process_entity(pair, repository),
            _ => (),
        }
    }

    let relationship = Relationship {
        from: source_entity_id,
        to: target_entity_id,
        relationship_type: link_id.to_string(),
        time: times,
        notes: None,
    };

    repository.add_relationship(relationship);
}

// Process link
pub fn process_link(pair: Pair<'_, Rule>) -> (String, Vec<Time>) {
    let mut link_id = "";
    let mut times: Vec<Time> = Vec::new();

    let inner_rules = pair.into_inner();
    for pair in inner_rules {
        match pair.as_rule() {
            Rule::id => link_id = pair.as_str(),
            Rule::time => times.push(process_time(pair).unwrap()),
            _ => (),
        }
    }

    (link_id.to_string(), times)
}

// Process an entity
pub fn process_entity(pair: Pair<'_, Rule>, repository: &mut Repository) -> EntityType {
    let mut entity_id = EntityType::Unknown;

    let inner_rules = pair.into_inner();

    for pair in inner_rules {
        entity_id = match pair.as_rule() {
            Rule::person => process_person(pair, repository),
            Rule::event => process_event(pair, repository),
            Rule::location => process_location(pair, repository),
            Rule::object => process_object(pair, repository),
            //Rule::alias => process_alias(pair, repository),
            // Rule::entity => process_entity(pair),
            // Rule::relationship => println!("Relationship"),
            _ => EntityType::Unknown, // TODO this is an error condition
        };
    }

    entity_id
}

// Process and event
pub fn process_event(pair: Pair<'_, Rule>, repository: &mut Repository) -> EntityType {
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

    let event = Event::new(name.to_string(), description, time);

    repository.add_event(event)
}
// Process an alias
pub fn process_alias(pair: Pair<'_, Rule>) -> Option<String> {
    println!("DEBUG: process_alias: {}", pair.as_str());

    let mut alias = None;

    let inner_rules = pair.into_inner();
    for pair in inner_rules {
        match pair.as_rule() {
            Rule::id => alias = Some(pair.as_str().to_string()),
            _ => (),
        }
    }

    alias
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
pub fn process_location(pair: Pair<'_, Rule>, repository: &mut Repository) -> EntityType {
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
    let location = Location::new(name.to_string(), description);

    repository.add_location(location)
}

// Process an object
pub fn process_object(pair: Pair<'_, Rule>, repository: &mut Repository) -> EntityType {
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
    let mut object = Object::new(name.to_string(), description);

    match object_qualifier {
        ObjectQualifier::None => (),
        ObjectQualifier::Weapon => object.set_type(crate::model::object::ObjectType::Weapon),
        _ => todo!(),
    }

    // TODO ObjectType and ObjectQualifier seem to be redundant

    repository.add_object(object)
}

pub fn process_person(pair: Pair<'_, Rule>, repository: &mut Repository) -> EntityType {
    let mut name = "";
    let mut person_qualifier = PersonQualifier::None;
    let mut alias = None;

    //TODO
    let notes: Option<String> = None;

    let inner_rules = pair.into_inner();
    for pair in inner_rules {
        match pair.as_rule() {
            Rule::id => name = pair.as_str(),
            Rule::person_qualifier => person_qualifier = PersonQualifier::from_str(pair.as_str()),
            Rule::alias => alias = process_alias(pair),

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

    if let Some(alias_name) = alias {
        person.add_alias(alias_name);
    };

    repository.add_person(person)
}

#[cfg(test)]
mod tests {
    use crate::model::relationship::Relationship;

    use super::*;

    use crate::model::EntityType;

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
    fn test_parse_alias() {
        let mut repo: Repository = Repository::new();
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

    #[test]
    fn test_parse_relationships() {
        let mut repo: Repository = Repository::new();
        TroyParser::build_model("pv Robert Bayly   body found at   l Quarry", &mut repo);

        assert_eq!(repo.persons.len(), 1);
        assert_eq!(repo.locations.len(), 1);
        assert_eq!(repo.relationships.len(), 1);

        let person = repo.persons.find("Robert Bayly");
        assert!(person.is_some());

        let person = person.unwrap();
        assert_eq!(person.name, "Robert Bayly");
        let person_id = person.id();

        let location = repo.locations.find("Quarry");
        assert!(location.is_some());
        let location_id = location.unwrap().id();

        let relationships: Vec<&Relationship> = repo.relationships.find(
            EntityType::Person(person_id),
            EntityType::Location(location_id),
        );
        assert!(relationships.len() == 1);

        let relationship = relationships[0];
        assert_eq!(relationship.relationship_type, "body found at");

        // Try a different order of the relationship entities
        let relationships: Vec<&Relationship> = repo.relationships.find(
            EntityType::Location(location_id),
            EntityType::Person(person_id),
        );
        assert!(relationships.len() == 1);

        let relationship = relationships[0];
        assert_eq!(relationship.relationship_type, "body found at");
    }

    #[test]
    fn test_parse_many_relationships() {
        let mut repo: Repository = Repository::new();
        TroyParser::build_model("pv Robert Bayly   body found at   l Quarry", &mut repo);
        TroyParser::build_model("pv Robert Bayly   owner   l Quarry", &mut repo);

        assert_eq!(repo.persons.len(), 1);
        assert_eq!(repo.locations.len(), 1);
        assert_eq!(repo.relationships.len(), 2);

        let person = repo.persons.find("Robert Bayly");
        assert!(person.is_some());

        let person_id = person.unwrap().id();

        let location = repo.locations.find("Quarry");
        assert!(location.is_some());
        let location_id = location.unwrap().id();

        let relationships: Vec<&Relationship> = repo.relationships.find(
            EntityType::Person(person_id),
            EntityType::Location(location_id),
        );
        assert!(relationships.len() == 2);

        let relationship = relationships[0];
        assert_eq!(relationship.relationship_type, "body found at");

        let relationship = relationships[1];
        assert_eq!(relationship.relationship_type, "owner");
    }

    #[test]
    fn test_relationship_times() {
        let mut repo: Repository = Repository::new();
        TroyParser::build_model(
            "pv Robert Bayly   married  t from 1984   t until 1999   p Rebecca Bayly",
            &mut repo,
        );

        let robert = repo.persons.find("Robert Bayly").unwrap();

        let rebecca = repo.persons.find("Rebecca Bayly").unwrap();

        let relationships: Vec<&Relationship> = repo.relationships.find(
            EntityType::Person(robert.id()),
            EntityType::Person(rebecca.id()),
        );
        assert!(relationships.len() == 1);

        assert_eq!(relationships[0].time.len(), 2);

        let times: Vec<String> = relationships[0].time.iter().map(|t| t.0.clone()).collect();

        assert!(times.contains(&"from 1984".to_string()));
        assert!(times.contains(&"until 1999".to_string()));
    }
}
