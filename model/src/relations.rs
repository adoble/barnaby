use std::collections::HashMap;
use crate::entity_type::EntityType;



#[derive(Debug, Clone)]
pub struct ObjectPersonRelation {
    pub object_id: u32,
    pub person_id: u32,
    pub relationship_type: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ObjectLocationRelation {
    pub object_id: u32,
    pub location_id: u32,
    pub placement: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PersonLocationRelation {
    pub person_id: u32,
    pub location_id: u32,
    pub role: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
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

#[derive(Debug, Default)]
pub struct Relations {
    pub object_person: HashMap<(u32, u32), ObjectPersonRelation>,
    pub object_location: HashMap<(u32, u32), ObjectLocationRelation>,
    pub person_location: HashMap<(u32, u32), PersonLocationRelation>,
    pub event_relations: Vec<EventRelation>,
}