use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
    Generic,
    Weapon,
    // Future types can be added here:
    // Evidence,
    // Document,
    // Vehicle,
    // etc.
}

#[derive(Debug, Clone)]
pub struct Object {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub object_type: ObjectType,
}

impl Object {
    pub fn new(id: u32, name: String, description: Option<String>) -> Self {
        Object {
            id,
            name,
            description,
            object_type: ObjectType::Generic,
        }
    }

    pub fn set_type(&mut self, object_type: ObjectType) {
        self.object_type = object_type;
    }

    pub fn is_weapon(&self) -> bool {
        self.object_type == ObjectType::Weapon
    }
}

#[derive(Debug, Default)]
pub struct Objects {
    items: HashMap<u32, Object>,
    next_id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_types() {
        let mut sword = Object::new(
            1,
            "Indian Sword".to_string(),
            Some("Stolen from vicarage".to_string()),
        );
        
        assert!(!sword.is_weapon());
        
        sword.set_type(ObjectType::Weapon);
        assert!(sword.is_weapon());
        
        let vase = Object::new(
            2,
            "Ming Vase".to_string(),
            None,
        );
        assert!(!vase.is_weapon());
    }
}