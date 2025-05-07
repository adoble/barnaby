use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Person {
    pub(super) id: u32,
    pub name: String,
    pub aliases: HashSet<String>,
    pub description: Option<String>,
    pub is_victim: bool,
}

impl Person {
    // Private constructor for Person. This is to restrict the creation of
    // ids to the Repository.
    fn new_with_id(id: u32, name: String, description: Option<String>) -> Self {
        Person {
            id,
            name,
            aliases: HashSet::new(),
            description,
            is_victim: false,
        }
    }

    /// Creates a new Person instance.
    ///
    /// # Arguments
    /// * `name` - Primary name of the person
    /// * `description` - Optional description of the person
    ///
    /// # Returns
    /// A new Person instance
    pub fn new(name: String, description: Option<String>) -> Self {
        Person {
            id: 0, // Temporary ID, will be replaced when added to Repository
            name,
            aliases: HashSet::new(),
            description,
            is_victim: false,
        }
    }

    // Getter for id
    pub fn id(&self) -> u32 {
        self.id
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

    /// Returns an iterator over the aliases
    pub fn aliases(&self) -> impl Iterator<Item = &String> {
        self.aliases.iter()
    }

    /// Returns the number of aliases
    pub fn number_aliases(&self) -> usize {
        self.aliases.len()
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
pub struct Persons(pub HashMap<u32, Person>);

impl Persons {
    /// Creates a new empty Persons collection
    pub fn new() -> Persons {
        Persons(HashMap::new())
    }

    /// Adds a person to the collection
    ///
    /// # Arguments
    /// * `person` - Person to add
    ///
    /// # Returns
    /// The ID of the added person
    pub fn add(&mut self, person: Person) -> u32 {
        let id = person.id;
        self.0.insert(person.id, person);
        id
    }

    /// Gets a mutable person by id
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Person> {
        self.0.get_mut(&id)
    }

    /// Finds a person by name or alias
    ///
    /// # Arguments
    /// * `name` - Name or alias to search for
    pub fn find(&self, name: &str) -> Option<&Person> {
        self.0.values().find(|p| p.matches_name(name))
    }

    /// Finds a person by name or alias and returns a mutable reference
    ///
    /// # Arguments
    /// * `name` - Name or alias to search for
    pub fn find_mut(&mut self, name: &str) -> Option<&mut Person> {
        self.0.values_mut().find(|p| p.matches_name(name))
    }

    /// Returns the number of persons in the collection
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the collection contains no persons
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_person() {
        let person = Person::new_with_id(
            1,
            "John Barnaby".to_string(),
            Some("Detective Chief Inspector".to_string()),
        );

        assert_eq!(person.id, 1);
        assert_eq!(person.name, "John Barnaby");
        assert_eq!(
            person.description,
            Some("Detective Chief Inspector".to_string())
        );

        // Test without description
        let person2 = Person::new_with_id(2, "Sarah Barnaby".to_string(), None);

        assert_eq!(person2.id, 2);
        assert_eq!(person2.name, "Sarah Barnaby");
        assert_eq!(person2.description, None);
    }

    #[test]
    fn test_person_with_aliases() {
        let mut person = Person::new_with_id(
            1,
            "Richard Bayly".to_string(),
            Some("Property developer".to_string()),
        );

        person.add_alias("Richard");
        person.add_alias("Rich");

        assert_eq!(person.name, "Richard Bayly");
        assert!(person.matches_name("Richard Bayly"));
        assert!(!person.matches_name("Richard Bay"));
        assert!(person.matches_name("Richard"));
        assert!(person.matches_name("Rich"));
        assert!(!person.matches_name("Dick"));

        // Test that aliases are unique
        person.add_alias("Richard"); // Adding same alias again
        assert_eq!(person.aliases.len(), 2);
    }

    #[test]
    fn test_midsomer_characters() {
        let mut tom = Person::new_with_id(
            1,
            "Thomas Barnaby".to_string(),
            Some("Former Detective Chief Inspector".to_string()),
        );
        tom.add_alias("Tom Barnaby");
        tom.add_alias("Tom");

        let mut joyce = Person::new_with_id(
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
        let mut person = Person::new_with_id(1, "Thomas Barnaby".to_string(), None);

        // Add aliases
        person.add_alias("Tom");
        person.add_alias("Tom"); // Duplicate - will be ignored
        person.add_alias("DCI Barnaby");

        assert_eq!(person.aliases.len(), 2); // Only unique aliases stored
        assert!(person.matches_name("Tom")); // Fast lookup
    }

    #[test]
    fn test_victim_status() {
        let mut person = Person::new_with_id(
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

    #[test]
    fn test_persons_collection() {
        let mut persons = Persons::new();
        let person = Person::new_with_id(1, "Tom Barnaby".to_string(), Some("DCI".to_string()));

        persons.0.insert(person.id, person);
        assert_eq!(persons.0.len(), 1);

        let retrieved = persons.0.get(&1).unwrap();
        assert_eq!(retrieved.name, "Tom Barnaby");
    }

    #[test]
    fn test_find_person() {
        let mut persons = Persons::new();
        let person = Person::new_with_id(1, "Tom Barnaby".to_string(), Some("DCI".to_string()));
        persons.add(person);

        let person = Person::new_with_id(2, "Ben Jones".to_string(), Some("DS".to_string()));
        persons.add(person);

        let person =
            Person::new_with_id(3, "Kate Wilding".to_string(), Some("Forensics".to_string()));
        persons.add(person);

        let person = persons.find("Tom Barnaby");

        // Find by primary name
        assert!(person.is_some());
        assert_eq!(persons.find("Tom Barnaby").unwrap().name, "Tom Barnaby");

        assert!(persons.find("Kate Wilding").is_some());
        assert_eq!(persons.find("Kate Wilding").unwrap().name, "Kate Wilding");
    }

    #[test]
    fn test_find_by_alias() {
        let mut persons = Persons::new();

        let mut tom = Person::new_with_id(1, "Thomas Barnaby".to_string(), Some("DCI".to_string()));
        tom.add_alias("Tom");
        tom.add_alias("DCI Barnaby");

        persons.add(tom);

        // Find by primary name
        assert_eq!(persons.find("Thomas Barnaby").unwrap().id, 1);
        // Find by aliases
        assert_eq!(persons.find("Tom").unwrap().id, 1);
        assert_eq!(persons.find("DCI Barnaby").unwrap().id, 1);
        // Non-existent name
        assert!(persons.find("John Nettles").is_none());
    }

    #[test]
    fn test_number_aliases() {
        let mut person = Person::new_with_id(1, "Tom Barnaby".to_string(), None);
        person.add_alias("Tom");
        person.add_alias("DCI Barnaby");

        assert_eq!(person.number_aliases(), 2);
    }

    #[test]
    fn test_aliases_iterator() {
        let mut person = Person::new_with_id(1, "Tom Barnaby".to_string(), None);
        person.add_alias("Tom");
        person.add_alias("DCI Barnaby");

        // Using HashSet as the order of the elements can be different
        let aliases: HashSet<String> = person.aliases().cloned().collect();
        let expected: HashSet<String> = vec!["Tom", "DCI Barnaby"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(aliases, expected);
    }

    #[test]
    fn test_persons_len() {
        let mut persons = Persons::new();
        assert_eq!(persons.len(), 0);
        assert!(persons.is_empty());

        persons.add(Person::new_with_id(
            1,
            "Tom Barnaby".to_string(),
            Some("DCI".to_string()),
        ));
        assert_eq!(persons.len(), 1);
        assert!(!persons.is_empty());

        persons.add(Person::new_with_id(2, "Joyce Barnaby".to_string(), None));
        assert_eq!(persons.len(), 2);
    }

    #[test]
    fn test_find_mut() {
        let mut persons = Persons::new();

        let mut tom = Person::new_with_id(1, "Thomas Barnaby".to_string(), Some("DCI".to_string()));
        tom.add_alias("Tom");
        persons.add(tom);

        // Find and modify the person
        if let Some(person) = persons.find_mut("Tom") {
            person.set_victim_status(true);
        }

        // Verify the modification
        assert!(persons.find("Thomas Barnaby").unwrap().is_victim);
    }

    #[test]
    fn test_add_returns_id() {
        let mut persons = Persons::new();

        let tom = Person::new_with_id(1, "Thomas Barnaby".to_string(), Some("DCI".to_string()));
        let id = persons.add(tom);
        assert_eq!(id, 1);

        let joyce = Person::new_with_id(2, "Joyce Barnaby".to_string(), None);
        let id = persons.add(joyce);
        assert_eq!(id, 2);

        // Verify both persons are in the collection
        assert!(persons.find("Thomas Barnaby").is_some());
        assert!(persons.find("Joyce Barnaby").is_some());
    }
}
