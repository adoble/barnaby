use std::collections::HashMap;
use super::entity_type::EntityType;
use super::time::Time;

#[derive(Debug, Clone)]
pub struct Relation {
    pub from: EntityType,
    pub to: EntityType,
    pub relationship_type: String,
    pub start_time: Option<Time>,
    pub end_time: Option<Time>,
    pub notes: Option<String>,
}


pub struct Relations (HashMap<(EntityType, EntityType), Relation>);


impl Relations {
    pub fn new() -> Self {
        Relations(HashMap::new())
    }

    pub fn add_relation(&mut self,  relation: Relation) {
        self.0.insert((relation.from,relation.to) , relation.clone());
    }

    pub fn get_relation(&self, from: EntityType, to: EntityType) -> Option<Relation> {
        self.0.get(&(from, to)).cloned()
    }

    pub fn remove_relation(&mut self, from: &EntityType, to: &EntityType) {
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
        let relation = Relation {
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
        let relation = Relation {
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
        let mut relations = Relations::new();

        // Test adding a relation
        let murder_weapon = Relation {
            from: EntityType::Object(1),
            to: EntityType::Location(2),
            relationship_type: "Found at".to_string(),
            start_time: Some(Time("morning".to_string())),
            end_time: None,
            notes: Some("Indian sword at vicarage".to_string()),
        };

        // Fixed line - removed unnecessary clone()
        relations.add_relation(murder_weapon);

        // Test retrieval
        let key = (EntityType::Object(1), EntityType::Location(2));
        let retrieved = relations.get_relation(EntityType::Object(1), EntityType::Location(2)).unwrap();
        assert_eq!(retrieved.relationship_type, "Found at");

        // Test removal
        relations.remove_relation(&EntityType::Object(1), &EntityType::Location(2));
        assert!(relations.get_relation(EntityType::Object(1), EntityType::Location(2)).is_none());
    }

    #[test]
    fn test_multiple_relations() {
        let mut relations = Relations::new();
        
        // Adjacent locations
        let vicarage_garden = Relation {
            from: EntityType::Location(1),
            to: EntityType::Location(2),
            relationship_type: "Adjacent to".to_string(),
            start_time: Some(Time("always".to_string())),
            end_time: None,
            notes: Some("Vicarage garden next to churchyard".to_string()),
        };

        // Event causality
        let murder_discovery = Relation {
            from: EntityType::Event(1),
            to: EntityType::Event(2),
            relationship_type: "Leads to".to_string(),
            start_time: None,
            end_time: None,
            notes: Some("Murder leads to discovery of family secret".to_string()),
        };

        relations.add_relation(vicarage_garden);
        relations.add_relation(murder_discovery);
        
        assert_eq!(relations.len(), 2);
    }
}