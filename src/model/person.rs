use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Person {
    pub id: u32,
    pub name: String,
    pub aliases: HashSet<String>,
    pub description: Option<String>,
    pub is_victim: bool,
}

impl Person {
    /// Creates a new Person instance.
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for the person
    /// * `name` - Primary name of the person
    /// * `description` - Optional description of the person
    /// 
    /// # Returns
    /// A new Person instance
    pub fn new(id: u32, name: String, description: Option<String>) -> Self {
        Person {
            id,
            name,
            aliases: HashSet::new(),
            description,
            is_victim: false,
        }
    }

    /// Adds an alias for the person
    /// 
    /// # Arguments
    /// * `alias` - Alternative name for the person
    pub fn add_alias<S: Into<String>>(&mut self, alias: S) {
        self.aliases.insert(alias.into());
    }

    /// Checks if a name matches either the primary name or any alias
    /// 
    /// # Arguments
    /// * `search_name` - Name to search for
    pub fn matches_name(&self, search_name: &str) -> bool {
        self.name == search_name || self.aliases.contains(search_name)
    }

    /// Sets the victim status for the person
    /// 
    /// # Arguments
    /// * `is_victim` - Boolean indicating if the person is a victim
    pub fn set_victim_status(&mut self, is_victim: bool) {
        self.is_victim = is_victim;
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
    fn test_person_with_aliases() {
        let mut person = Person::new(
            1,
            "Richard Bayly".to_string(),
            Some("Property developer".to_string()),
        );

        person.add_alias("Richard");
        person.add_alias("Rich");

        assert_eq!(person.name, "Richard Bayly");
        assert!(person.matches_name("Richard"));
        assert!(person.matches_name("Rich"));
        assert!(!person.matches_name("Dick"));
        
        // Test that aliases are unique
        person.add_alias("Richard"); // Adding same alias again
        assert_eq!(person.aliases.len(), 2);
    }

    #[test]
    fn test_midsomer_characters() {
        let mut tom = Person::new(
            1,
            "Thomas Barnaby".to_string(),
            Some("Former Detective Chief Inspector".to_string()),
        );
        tom.add_alias("Tom Barnaby");
        tom.add_alias("Tom");

        let mut joyce = Person::new(
            2,
            "Joyce Barnaby".to_string(),
            Some("Tom's wife and excellent cook".to_string()),
        );
        joyce.add_alias("Mrs. Barnaby");

        assert!(tom.matches_name("Tom"));
        assert!(tom.matches_name("Thomas Barnaby"));
        assert!(joyce.matches_name("Mrs. Barnaby"));
        assert!(!joyce.matches_name("Joyce"));
    }

    #[test]
    fn test_alias_uniqueness_and_lookup() {
        let mut person = Person::new(
            1,
            "Thomas Barnaby".to_string(),
            None,
        );
        
        // Add aliases
        person.add_alias("Tom");
        person.add_alias("Tom");  // Duplicate - will be ignored
        person.add_alias("DCI Barnaby");
        
        assert_eq!(person.aliases.len(), 2);  // Only unique aliases stored
        assert!(person.matches_name("Tom"));  // Fast lookup
    }

    #[test]
    fn test_victim_status() {
        let mut person = Person::new(
            1,
            "Richard Bayly".to_string(),
            Some("Property developer".to_string()),
        );

        assert!(!person.is_victim);
        
        person.set_victim_status(true);
        assert!(person.is_victim);
        
        person.set_victim_status(false);
        assert!(!person.is_victim);
    }
}