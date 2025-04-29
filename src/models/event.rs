use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub datetime: String,
}

#[derive(Debug, Default)]
pub struct Events {
    items: HashMap<u32, Event>,
    next_id: u32,
}