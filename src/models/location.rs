use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Default)]
pub struct Locations {
    items: HashMap<u32, Location>,
    next_id: u32,
}