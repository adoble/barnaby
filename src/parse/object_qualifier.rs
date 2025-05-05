#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectQualifier {
    None,
    Weapon,
    Evidence,
    Document,
    Vehicle,
    Other(String),
}

impl ObjectQualifier {
    pub fn from_str(s: &str) -> Self {
        match s {
            "" => ObjectQualifier::None,
            "w" | "W" => ObjectQualifier::Weapon,
            "e" | "E" => ObjectQualifier::Evidence,
            "d" | "D" => ObjectQualifier::Document,
            "v" | "V" => ObjectQualifier::Vehicle,
            _ => ObjectQualifier::Other(s.to_string()),
        }
    }
}

impl Default for ObjectQualifier {
    fn default() -> Self {
        ObjectQualifier::None
    }
}
impl std::fmt::Display for ObjectQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectQualifier::None => write!(f, ""),
            ObjectQualifier::Weapon => write!(f, "w"),
            ObjectQualifier::Evidence => write!(f, "e"),
            ObjectQualifier::Document => write!(f, "d"),
            ObjectQualifier::Vehicle => write!(f, "v"),
            ObjectQualifier::Other(s) => write!(f, "{}", s),
        }
    }
}
