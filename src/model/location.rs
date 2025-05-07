use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Location {
    pub(super) id: u32,
    pub name: String,
    pub description: Option<String>,
}

impl Location {
    // Private constructor. This is to restrict the creation of
    // ids to the repository
    fn new_with_id(id: u32, name: String, description: Option<String>) -> Self {
        Location {
            id,
            name,
            description,
        }
    }
    /// Creates a new Location instance.
    ///
    /// # Arguments
    /// * `name` - Name of the location
    /// * `description` - Optional description of the location
    ///
    /// # Returns
    /// A new Location instance
    pub fn new(name: String, description: Option<String>) -> Self {
        Location {
            id: 0,
            name,
            description,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Default)]
pub struct Locations(pub HashMap<u32, Location>);

impl Locations {
    /// Creates a new empty Locations collection
    pub fn new() -> Self {
        Locations(HashMap::new())
    }

    /// Adds a location to the collection
    ///
    /// # Arguments
    /// * `location` - Location to add
    /// Returns the id of the added location
    pub fn add(&mut self, location: Location) -> u32 {
        let id = location.id;
        self.0.insert(location.id, location);
        id
    }

    /// Returns the number of locations in the collection
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the collection contains no locations
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Finds a location by name
    pub fn find(&self, name: &str) -> Option<&Location> {
        self.0.values().find(|loc| loc.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_location() {
        let location = Location::new_with_id(
            1,
            "Badger's Drift".to_string(),
            Some("Small village in Midsomer County".to_string()),
        );

        assert_eq!(location.id, 1);
        assert_eq!(location.name, "Badger's Drift");
        assert_eq!(
            location.description,
            Some("Small village in Midsomer County".to_string())
        );
    }

    #[test]
    fn test_locations_collection() {
        let mut locations = Locations::new();

        let vicarage = Location::new_with_id(
            1,
            "St. Michael's Vicarage".to_string(),
            Some("Where the Indian sword was stolen".to_string()),
        );

        locations.add(vicarage);
        assert_eq!(locations.0.len(), 1);

        let retrieved = locations.0.get(&1).unwrap();
        assert_eq!(retrieved.name, "St. Michael's Vicarage");
    }

    #[test]
    fn test_multiple_locations() {
        let mut locations = Locations::new();

        let vicarage = Location::new_with_id(1, "St. Michael's Vicarage".to_string(), None);

        let village_fete = Location::new_with_id(
            2,
            "Village Fete".to_string(),
            Some("Annual gathering, scene of the archery incident".to_string()),
        );

        locations.add(vicarage);
        locations.add(village_fete);

        assert_eq!(locations.0.len(), 2);
        assert!(locations.0.contains_key(&1));
        assert!(locations.0.contains_key(&2));
    }

    #[test]
    fn test_locations_len_and_find() {
        let mut locations = Locations::new();
        assert_eq!(locations.len(), 0);
        assert!(locations.is_empty());

        let badgers_drift = Location::new_with_id(
            1,
            "Badger's Drift".to_string(),
            Some("Small village in Midsomer County".to_string()),
        );
        locations.add(badgers_drift);

        assert_eq!(locations.len(), 1);
        assert!(!locations.is_empty());

        // Test find functionality
        let found = locations.find("Badger's Drift");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, 1);

        // Test non-existent location
        assert!(locations.find("Midsomer Worthy").is_none());
    }
}
