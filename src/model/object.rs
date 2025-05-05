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
pub struct Objects(HashMap<u32, Object>);

impl Objects {
    pub fn new() -> Objects {
        Objects(HashMap::new())
    }

    pub fn add(&mut self, object: Object) {
        self.0.insert(object.id, object);
    }
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

        let vase = Object::new(2, "Ming Vase".to_string(), None);
        assert!(!vase.is_weapon());
    }

    #[test]
    fn test_objects_collection() {
        let mut objects = Objects::new();

        let sword = Object::new(
            1,
            "Indian Sword".to_string(),
            Some("Murder weapon stolen from vicarage".to_string()),
        );

        objects.add(sword);
        assert_eq!(objects.0.len(), 1);

        let retrieved = objects.0.get(&1).unwrap();
        assert_eq!(retrieved.name, "Indian Sword");
        assert!(!retrieved.is_weapon()); // Starts as Generic type
    }

    #[test]
    fn test_multiple_objects() {
        let mut objects = Objects::new();

        let mut sword = Object::new(
            1,
            "Indian Sword".to_string(),
            Some("Murder weapon".to_string()),
        );
        sword.set_type(ObjectType::Weapon);

        let arrow = Object::new(
            2,
            "Arrow".to_string(),
            Some("Found at village fete".to_string()),
        );

        objects.add(sword);
        objects.add(arrow);

        assert_eq!(objects.0.len(), 2);
        assert!(objects.0.contains_key(&1));
        assert!(objects.0.contains_key(&2));

        // Verify object types are maintained
        assert!(objects.0.get(&1).unwrap().is_weapon());
        assert!(!objects.0.get(&2).unwrap().is_weapon());
    }
}
