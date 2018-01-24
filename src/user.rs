use std::fmt;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }

    pub fn to_string(&self) -> String {
        let &UserId(ref inner) = self;
        inner.to_string()
    }

    pub fn from_string(s: String) -> Option<Self> {
        Uuid::parse_str(&s).map(|id| UserId(id)).ok()
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}

