#[derive(Debug, Clone, PartialEq)]
pub enum PersonQualifier {
    None,
    Victim,
    Witness,
    Suspect,
    Other(String),
}
impl PersonQualifier {
    pub fn from_str(s: &str) -> Self {
        match s {
            "" => PersonQualifier::None,
            "v" | "V" => PersonQualifier::Victim,
            "w" | "W" => PersonQualifier::Witness,
            "s" | "S" => PersonQualifier::Suspect,
            _ => PersonQualifier::Other(s.to_string()),
        }
    }
}
