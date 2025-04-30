#[derive(Debug, Clone, PartialEq)]
pub struct Time(pub String);

impl Time {
    pub fn new<S: Into<String>>(description: S) -> Self {
        Time(description.into())
    }
}