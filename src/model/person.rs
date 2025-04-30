use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

impl Person {
    /// Creates a new Person instance.
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for the person
    /// * `name` - Name of the person
    /// * `description` - Optional description of the person
    /// 
    /// # Returns
    /// A new Person instance
    pub fn new(id: u32, name: String, description: Option<String>) -> Self {
        Person {
            id,
            name,
            description,
        }
    }
}

#[derive(Debug, Default)]
pub struct Persons {
    items: HashMap<u32, Person>,
    next_id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_person() {
        let person = Person::new(
            1,
            "John Barnaby".to_string(),
            Some("Detective Chief Inspector".to_string()),
        );

        assert_eq!(person.id, 1);
        assert_eq!(person.name, "John Barnaby");
        assert_eq!(person.description, Some("Detective Chief Inspector".to_string()));
        
        // Test without description
        let person2 = Person::new(
            2,
            "Sarah Barnaby".to_string(),
            None,
        );
        
        assert_eq!(person2.id, 2);
        assert_eq!(person2.name, "Sarah Barnaby");
        assert_eq!(person2.description, None);
    }

    #[test]
    fn test_midsomer_characters() {
        let tom = Person::new(
            1,
            "Tom Barnaby".to_string(),
            Some("Former Detective Chief Inspector".to_string()),
        );

        let ben = Person::new(
            2,
            "Ben Jones".to_string(),
            Some("Detective Sergeant".to_string()),
        );

        let joyce = Person::new(
            3,
            "Joyce Barnaby".to_string(),
            Some("Tom's wife and excellent cook".to_string()),
        );

        let winter = Person::new(
            4,
            "Jamie Winter".to_string(),
            Some("Detective Sergeant under John Barnaby".to_string()),
        );

        assert_eq!(tom.name, "Tom Barnaby");
        assert_eq!(ben.description, Some("Detective Sergeant".to_string()));
        assert_eq!(joyce.name, "Joyce Barnaby");
        assert_eq!(winter.description, Some("Detective Sergeant under John Barnaby".to_string()));
    }
}