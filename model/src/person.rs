use std::collections::HashMap;



#[derive(Debug, Clone)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Default)]
pub struct Persons {
    items: HashMap<u32, Person>,
    next_id: u32,
}