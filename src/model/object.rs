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
    pub(super) id: u32,
    pub name: String,
    pub description: Option<String>,
    pub object_type: ObjectType,
}

impl Object {
    pub fn new_with_id(id: u32, name: String, description: Option<String>) -> Self {
        Object {
            id,
            name,
            description,
            object_type: ObjectType::Generic,
        }
    }
    pub fn new(name: String, description: Option<String>) -> Self {
        Object {
            id: 0, // Placeholder for ID, to be set later
            name,
            description,
            object_type: ObjectType::Generic,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
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

    pub fn add(&mut self, object: Object) -> u32 {
        let id = object.id;
        self.0.insert(object.id, object);
        id
    }

    /// Returns the number of objects in the collection
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the collection contains no objects
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Returns an immutable object by id
    pub fn get(&self, id: u32) -> Option<&Object> {
        self.0.get(&id)
    }

    /// Finds an object by name
    pub fn find(&self, name: &str) -> Option<&Object> {
        self.0.values().find(|obj| obj.name == name)
    }

    /// Returns an iterator over the objects in the collection
    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.0.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_types() {
        let mut sword = Object::new_with_id(
            1,
            "Indian Sword".to_string(),
            Some("Stolen from vicarage".to_string()),
        );

        assert!(!sword.is_weapon());

        sword.set_type(ObjectType::Weapon);
        assert!(sword.is_weapon());

        let vase = Object::new_with_id(2, "Ming Vase".to_string(), None);
        assert!(!vase.is_weapon());
    }

    #[test]
    fn test_objects_collection() {
        let mut objects = Objects::new();

        let sword = Object::new_with_id(
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

        let mut sword = Object::new_with_id(
            1,
            "Indian Sword".to_string(),
            Some("Murder weapon".to_string()),
        );
        sword.set_type(ObjectType::Weapon);

        let arrow = Object::new_with_id(
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

    #[test]
    fn test_objects_len_and_find() {
        let mut objects = Objects::new();
        assert_eq!(objects.len(), 0);
        assert!(objects.is_empty());

        let sword = Object::new_with_id(
            1,
            "Indian Sword".to_string(),
            Some("Murder weapon".to_string()),
        );
        objects.add(sword);

        assert_eq!(objects.len(), 1);
        assert!(!objects.is_empty());

        // Test find functionality
        let found = objects.find("Indian Sword");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, 1);

        // Test non-existent object
        assert!(objects.find("Garden Shears").is_none());
    }

    #[test]
    fn test_objects_iterator() {
        let mut objects = Objects::new();

        let sword = Object::new_with_id(
            1,
            "Indian Sword".to_string(),
            Some("Murder weapon".to_string()),
        );
        let arrow = Object::new_with_id(
            2,
            "Arrow".to_string(),
            Some("Found at village fete".to_string()),
        );

        objects.add(sword);
        objects.add(arrow);

        let names: Vec<&str> = objects.iter().map(|obj| obj.name.as_str()).collect();

        assert_eq!(names.len(), 2);
        assert!(names.contains(&"Indian Sword"));
        assert!(names.contains(&"Arrow"));
    }
}
