use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Object {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Default)]
pub struct Objects {
    items: HashMap<u32, Object>,
    next_id: u32,
}