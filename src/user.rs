use std::fmt;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use serde::{Serialize, Serializer};

#[derive(Debug, Clone)]
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

impl Serialize for UserId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let &UserId(ref inner) = self;
        if serializer.is_human_readable() {
            serializer.collect_str(&inner.hyphenated())
        } else {
            serializer.serialize_bytes(inner.as_bytes())
        }
    }
}
