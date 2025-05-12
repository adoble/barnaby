use super::entity_type::EntityType;
use super::time::Time;

#[derive(Debug, Clone)]
pub struct Relationship {
    pub from: EntityType,
    pub to: EntityType,
    pub relationship_type: String,
    pub time: Vec<Time>,
    pub notes: Option<String>,
}

/// Represents a collection of relationships.
///
/// This approach uses a Vec rather than a HashMap as the possible theoretical performance benefits
/// of HashMap lookup have been traded for simpler code and more natural relationship handling.
/// As we will not be dealing with  thousands of relationships, the performance difference should
/// be negligible.
#[derive(Debug, Default)]
pub struct Relationships(Vec<Relationship>);

impl Relationships {
    pub fn new() -> Self {
        Relationships(Vec::new())
    }

    pub fn add(&mut self, relation: Relationship) {
        self.0.push(relation);
    }

    pub fn get(&self, from: EntityType, to: EntityType) -> Option<&Relationship> {
        self.0
            .iter()
            .find(|r| (r.from == from && r.to == to) || (r.from == to && r.to == from))
    }

    pub fn remove(&mut self, from: &EntityType, to: &EntityType) {
        self.0
            .retain(|r| !((r.from == *from && r.to == *to) || (r.from == *to && r.to == *from)));
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the relationship in the collection
    pub fn iter(&self) -> impl Iterator<Item = &Relationship> {
        self.0.iter()
    }

    /// Find all relationships between two entities.
    /// The order of the entities does not matter.
    ///
    /// Returns a vector of references to the relationships.
    /// If no relationships are found, an empty vector is returned.
    pub fn find(&self, entity_id_1: EntityType, entity_id_2: EntityType) -> Vec<&Relationship> {
        self.0
            .iter()
            .filter(|r| {
                (r.from == entity_id_1 && r.to == entity_id_2)
                    || (r.from == entity_id_2 && r.to == entity_id_1)
            })
            .collect()
    }

    fn id_raw(id: EntityType) -> u32 {
        match id {
            EntityType::Person(id) => id,
            EntityType::Object(id) => id,
            EntityType::Location(id) => id,
            EntityType::Event(id) => id,
            EntityType::Unknown => panic!("Unknown entity type"), // TODO handle this better
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::location::Location;

    use super::*;

    #[test]
    fn test_marriage_relation() {
        let relation = Relationship {
            from: EntityType::Person(1),
            to: EntityType::Person(2),
            relationship_type: "Married to".to_string(),
            time: vec![Time("20 years ago".to_string())],
            notes: Some("Tom and Joyce Barnaby".to_string()),
        };

        assert_eq!(relation.relationship_type, "Married to");
        assert_eq!(relation.time[0].0, "20 years ago");
    }

    #[test]
    fn test_weapon_location() {
        let relation = Relationship {
            from: EntityType::Object(1),
            to: EntityType::Location(3),
            relationship_type: "Hidden in".to_string(),
            time: vec![
                Time("night of the murder".to_string()),
                Time("discovery by police".to_string()),
            ],
            notes: Some("Murder weapon under floorboards".to_string()),
        };

        assert_eq!(relation.time.len(), 2);
        assert_eq!(relation.time[0].0, "night of the murder");
        assert_eq!(relation.time[1].0, "discovery by police");
    }

    #[test]
    fn test_relations_crud() {
        let mut relations = Relationships::new();

        // Test adding a relation
        let murder_weapon = Relationship {
            from: EntityType::Object(1),
            to: EntityType::Location(2),
            relationship_type: "Found at".to_string(),
            time: vec![Time("morning".to_string())],
            notes: Some("Indian sword at vicarage".to_string()),
        };

        relations.add(murder_weapon);

        // Test retrieval
        let retrieved = relations
            .get(EntityType::Object(1), EntityType::Location(2))
            .unwrap();
        assert_eq!(retrieved.relationship_type, "Found at");
        assert_eq!(retrieved.time[0].0, "morning");

        // Test removal
        relations.remove(&EntityType::Object(1), &EntityType::Location(2));
        assert!(relations
            .get(EntityType::Object(1), EntityType::Location(2))
            .is_none());
    }

    #[test]
    fn test_find_relationships() {
        let mut relationships = Relationships::new();

        let mut manor_house = Location::new("Manor house".to_string(), None);
        let mut farm = Location::new("Farm".to_string(), None);

        manor_house.id = 3;
        farm.id = 4;

        // Adjacent locations
        let land_adj = Relationship {
            from: EntityType::Location(manor_house.id()),
            to: EntityType::Location(farm.id()),
            relationship_type: "adjacent to".to_string(),
            time: vec![Time("always".to_string())],
            notes: None,
        };

        // Ownership relationship with multiple time points
        let land_ownership = Relationship {
            from: EntityType::Location(manor_house.id()),
            to: EntityType::Location(farm.id()),
            relationship_type: "owned by".to_string(),
            time: vec![
                Time("since middle ages".to_string()),
                Time("recorded in Domesday Book".to_string()),
            ],
            notes: None,
        };

        relationships.add(land_ownership);
        relationships.add(land_adj);

        assert_eq!(relationships.len(), 2);

        let found_relationships = relationships.find(
            EntityType::Location(manor_house.id()),
            EntityType::Location(farm.id()),
        );

        assert_eq!(found_relationships.len(), 2);

        // Check relationship types
        let found_relationship_names: Vec<String> = found_relationships
            .iter()
            .map(|r| r.relationship_type.clone())
            .collect();

        assert!(found_relationship_names.contains(&"adjacent to".to_string()));
        assert!(found_relationship_names.contains(&"owned by".to_string()));

        // Check time points
        let ownership = found_relationships
            .iter()
            .find(|r| r.relationship_type == "owned by")
            .unwrap();
        assert_eq!(ownership.time.len(), 2);
        assert_eq!(ownership.time[0].0, "since middle ages");
        assert_eq!(ownership.time[1].0, "recorded in Domesday Book");
    }
}
