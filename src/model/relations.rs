use std::collections::HashMap;
use super::entity_type::EntityType;
use super::time::Time;

#[derive(Debug, Clone)]
pub struct ObjectPersonRelation {
    pub object_id: u32,
    pub person_id: u32,
    pub relationship_type: String,
    pub start_date: Option<Time>,
    pub end_date: Option<Time>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ObjectLocationRelation {
    pub object_id: u32,
    pub location_id: u32,
    pub placement: Option<String>,
    pub since: Option<Time>,
    pub until: Option<Time>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PersonLocationRelation {
    pub person_id: u32,
    pub location_id: u32,
    pub role: Option<String>,
    pub start_date: Option<Time>,
    pub end_date: Option<Time>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EventRelation {
    pub event_id: u32,
    pub entity_id: u32,
    pub entity_type: EntityType,
    pub role: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ObjectObjectRelation {
    pub object_id_from: u32,
    pub object_id_to: u32,
    pub relationship_type: String,
    pub since: Option<Time>,
    pub until: Option<Time>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PersonPersonRelation {
    pub person_id_from: u32,
    pub person_id_to: u32,
    pub relationship_type: String,
    pub start_date: Option<Time>,
    pub end_date: Option<Time>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EventEventRelation {
    pub event_id_from: u32,
    pub event_id_to: u32,
    pub relationship_type: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LocationLocationRelation {
    pub location_id_from: u32,
    pub location_id_to: u32,
    pub relationship_type: String,
    pub since: Option<Time>,
    pub until: Option<Time>,
    pub notes: Option<String>,
}

#[derive(Debug, Default)]
pub struct Relations {
    pub object_person: HashMap<(u32, u32), ObjectPersonRelation>,
    pub object_location: HashMap<(u32, u32), ObjectLocationRelation>,
    pub person_location: HashMap<(u32, u32), PersonLocationRelation>,
    pub object_object: HashMap<(u32, u32), ObjectObjectRelation>,
    pub person_person: HashMap<(u32, u32), PersonPersonRelation>,
    pub event_event: HashMap<(u32, u32), EventEventRelation>,
    pub location_location: HashMap<(u32, u32), LocationLocationRelation>,
    pub event_relations: Vec<EventRelation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_person_relation() {
        let relation = PersonPersonRelation {
            person_id_from: 1,
            person_id_to: 2,
            relationship_type: "Married to".to_string(),
            start_date: Some(Time("20 years ago".to_string())),
            end_date: None,
            notes: Some("Tom and Joyce Barnaby".to_string()),
        };

        assert_eq!(relation.relationship_type, "Married to");
    }

    #[test]
    fn test_object_object_relation() {
        let relation = ObjectObjectRelation {
            object_id_from: 1,
            object_id_to: 2,
            relationship_type: "Contains".to_string(),
            since: Some(Time("when found".to_string())),
            until: None,
            notes: Some("Murder weapon inside garden shed".to_string()),
        };

        assert_eq!(relation.relationship_type, "Contains");
    }

    #[test]
    fn test_event_event_relation() {
        let relation = EventEventRelation {
            event_id_from: 1,
            event_id_to: 2,
            relationship_type: "Leads to".to_string(),
            notes: Some("Murder leads to discovery of old family secret".to_string()),
        };

        assert_eq!(relation.relationship_type, "Leads to");
    }

    #[test]
    fn test_location_location_relation() {
        let relation = LocationLocationRelation {
            location_id_from: 1,
            location_id_to: 2,
            relationship_type: "Adjacent to".to_string(),
            since: Some(Time("always".to_string())),
            until: None,
            notes: Some("Vicarage garden adjacent to churchyard".to_string()),
        };

        assert_eq!(relation.relationship_type, "Adjacent to");
    }

    #[test]
    fn test_relations_crud() {
        let mut relations = Relations::default();

        // Test Person-Person relationship
        let tom_joyce = PersonPersonRelation {
            person_id_from: 1,
            person_id_to: 2,
            relationship_type: "Married to".to_string(),
            start_date: Some(Time("20 years ago".to_string())),
            end_date: None,
            notes: Some("Tom and Joyce Barnaby".to_string()),
        };
        relations.person_person.insert((1, 2), tom_joyce.clone());
        assert_eq!(relations.person_person.get(&(1, 2)).unwrap().relationship_type, "Married to");

        // Test Event-Event relationship
        let murder_discovery = EventEventRelation {
            event_id_from: 1,
            event_id_to: 2,
            relationship_type: "Leads to".to_string(),
            notes: Some("Murder leads to secret discovery".to_string()),
        };
        relations.event_event.insert((1, 2), murder_discovery.clone());
        assert_eq!(relations.event_event.get(&(1, 2)).unwrap().relationship_type, "Leads to");

        // Test Object-Location relationship
        let weapon_shed = ObjectLocationRelation {
            object_id: 1,
            location_id: 3,
            placement: Some("Hidden under floorboards".to_string()),
            since: Some(Time("night of the murder".to_string())),
            until: Some(Time("discovery by police".to_string())),
            notes: Some("Murder weapon location".to_string()),
        };
        relations.object_location.insert((1, 3), weapon_shed.clone());
        assert_eq!(
            relations.object_location.get(&(1, 3)).unwrap().placement,
            Some("Hidden under floorboards".to_string())
        );

        // Test generic Event relation
        let murder_suspect = EventRelation {
            event_id: 1,
            entity_id: 4,
            entity_type: EntityType::Person,
            role: "Suspect".to_string(),
            notes: Some("Primary suspect in murder".to_string()),
        };
        relations.event_relations.push(murder_suspect.clone());
        assert_eq!(relations.event_relations[0].role, "Suspect");

        // Test removal
        relations.person_person.remove(&(1, 2));
        assert!(relations.person_person.get(&(1, 2)).is_none());
    }

    #[test]
    fn test_relations_empty() {
        let relations = Relations::default();
        assert!(relations.person_person.is_empty());
        assert!(relations.event_event.is_empty());
        assert!(relations.object_location.is_empty());
        assert!(relations.event_relations.is_empty());
    }

    #[test]
    fn test_relations_multiple() {
        let mut relations = Relations::default();
        
        // Add multiple location relationships
        let vicarage_garden = LocationLocationRelation {
            location_id_from: 1,
            location_id_to: 2,
            relationship_type: "Adjacent to".to_string(),
            since: Some(Time("always".to_string())),
            until: None,
            notes: Some("Vicarage garden next to churchyard".to_string()),
        };

        let church_village = LocationLocationRelation {
            location_id_from: 2,
            location_id_to: 3,
            relationship_type: "Part of".to_string(),
            since: Some(Time("medieval times".to_string())),
            until: None,
            notes: Some("Church in Midsomer village".to_string()),
        };

        relations.location_location.insert((1, 2), vicarage_garden);
        relations.location_location.insert((2, 3), church_village);

        assert_eq!(relations.location_location.len(), 2);
        assert!(relations.location_location.contains_key(&(1, 2)));
        assert!(relations.location_location.contains_key(&(2, 3)));
    }
}