#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum EntityType {
    Person(u32),
    Object(u32),
    Location(u32),
    Event(u32),
    Unknown,
}
