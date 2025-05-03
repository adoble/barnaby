#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum EntityType {
    Person(u32),
    Object(u32),
    Location(u32),
    Event(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_types() {
        let person = EntityType::Person(1);
        let object = EntityType::Object(2);
        let location = EntityType::Location(3);
        let event = EntityType::Event(4);

        match person {
            EntityType::Person(id) => assert_eq!(id, 1),
            _ => panic!("Wrong entity type"),
        }

        match object {
            EntityType::Object(id) => assert_eq!(id, 2),
            _ => panic!("Wrong entity type"),
        }
    }
}