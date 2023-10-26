use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, Debug)]
pub struct UUID(pub Uuid);

impl UUID {
    pub fn is_str_valid(value: &str) -> bool {
        Uuid::from_str(value).is_ok()
    }

    pub fn new_random() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Into<String> for UUID {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl From<String> for UUID {
    fn from(value: String) -> Self {
        Self(Uuid::from_str(&value).unwrap())
    }
}
