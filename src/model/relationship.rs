use super::entity_type::EntityType;
use super::time::Time;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Relationship {
    pub from: EntityType,
    pub to: EntityType,
    pub relationship_type: String,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
    pub notes: Option<String>,
}

pub struct Relationships(HashMap<(EntityType, EntityType), Relationship>);

impl Relationships {
    pub fn new() -> Self {
        Relationships(HashMap::new())
    }

    pub fn add(&mut self, relation: Relationship) {
        self.0
            .insert((relation.from, relation.to), relation.clone());
    }

    pub fn get(&self, from: EntityType, to: EntityType) -> Option<Relationship> {
        self.0.get(&(from, to)).cloned()
    }

    pub fn remove(&mut self, from: &EntityType, to: &EntityType) {
        self.0.remove(&(from.clone(), to.clone()));
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// [derive(Debug, Default, Eq, PartialEq)]
// pub struct Relations {
//     pub items: HashMap<(EntityType, EntityType), Relation>,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marriage_relation() {
        let relation = Relationship {
            from: EntityType::Person(1),
            to: EntityType::Person(2),
            relationship_type: "Married to".to_string(),
            start_time: Some(Time("20 years ago".to_string())),
            end_time: None,
            notes: Some("Tom and Joyce Barnaby".to_string()),
        };

        assert_eq!(relation.relationship_type, "Married to");
    }

    #[test]
    fn test_weapon_location() {
        let relation = Relationship {
            from: EntityType::Object(1),
            to: EntityType::Location(3),
            relationship_type: "Hidden in".to_string(),
            start_time: Some(Time("night of the murder".to_string())),
            end_time: Some(Time("discovery by police".to_string())),
            notes: Some("Murder weapon under floorboards".to_string()),
        };

        match relation.from {
            EntityType::Object(id) => assert_eq!(id, 1),
            _ => panic!("Wrong entity type"),
        }
    }

    #[test]
    fn test_relations_crud() {
        let mut relations = Relationships::new();

        // Test adding a relation
        let murder_weapon = Relationship {
            from: EntityType::Object(1),
            to: EntityType::Location(2),
            relationship_type: "Found at".to_string(),
            start_time: Some(Time("morning".to_string())),
            end_time: None,
            notes: Some("Indian sword at vicarage".to_string()),
        };

        // Fixed line - removed unnecessary clone()
        relations.add(murder_weapon);

        // Test retrieval
        let retrieved = relations
            .get(EntityType::Object(1), EntityType::Location(2))
            .unwrap();
        assert_eq!(retrieved.relationship_type, "Found at");

        // Test removal
        relations.remove(&EntityType::Object(1), &EntityType::Location(2));
        assert!(relations
            .get(EntityType::Object(1), EntityType::Location(2))
            .is_none());
    }

    #[test]
    fn test_multiple_relations() {
        let mut relations = Relationships::new();

        // Adjacent locations
        let vicarage_garden = Relationship {
            from: EntityType::Location(1),
            to: EntityType::Location(2),
            relationship_type: "Adjacent to".to_string(),
            start_time: Some(Time("always".to_string())),
            end_time: None,
            notes: Some("Vicarage garden next to churchyard".to_string()),
        };

        // Event causality
        let murder_discovery = Relationship {
            from: EntityType::Event(1),
            to: EntityType::Event(2),
            relationship_type: "Leads to".to_string(),
            start_time: None,
            end_time: None,
            notes: Some("Murder leads to discovery of family secret".to_string()),
        };

        relations.add(vicarage_garden);
        relations.add(murder_discovery);

        assert_eq!(relations.len(), 2);
    }
}
