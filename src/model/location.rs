use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
}

impl Location {
    /// Creates a new Location instance.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the location
    /// * `name` - Name of the location
    /// * `description` - Optional description of the location
    ///
    /// # Returns
    /// A new Location instance
    pub fn new(id: u32, name: String, description: Option<String>) -> Self {
        Location {
            id,
            name,
            description,
        }
    }
}

#[derive(Debug, Default)]
pub struct Locations {
    items: HashMap<u32, Location>,
    next_id: u32,
}

impl Locations {
    /// Creates a new empty Locations collection
    pub fn new() -> Self {
        Locations {
            items: HashMap::new(),
            next_id: 1,
        }
    }

    /// Adds a location to the collection
    ///
    /// # Arguments
    /// * `location` - Location to add
    pub fn add(&mut self, location: Location) {
        self.items.insert(location.id, location);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_location() {
        let location = Location::new(
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

        let vicarage = Location::new(
            1,
            "St. Michael's Vicarage".to_string(),
            Some("Where the Indian sword was stolen".to_string()),
        );

        locations.add(vicarage);
        assert_eq!(locations.items.len(), 1);

        let retrieved = locations.items.get(&1).unwrap();
        assert_eq!(retrieved.name, "St. Michael's Vicarage");
    }

    #[test]
    fn test_multiple_locations() {
        let mut locations = Locations::new();

        let vicarage = Location::new(1, "St. Michael's Vicarage".to_string(), None);

        let village_fete = Location::new(
            2,
            "Village Fete".to_string(),
            Some("Annual gathering, scene of the archery incident".to_string()),
        );

        locations.add(vicarage);
        locations.add(village_fete);

        assert_eq!(locations.items.len(), 2);
        assert!(locations.items.contains_key(&1));
        assert!(locations.items.contains_key(&2));
    }
}
