pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuerySupportMessagesForThreadResponseItemAuthor {
    Customer,
    Agent,
    System,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QuerySupportMessagesForThreadResponseItemAuthor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Customer => serializer.serialize_str("customer"),
            Self::Agent => serializer.serialize_str("agent"),
            Self::System => serializer.serialize_str("system"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QuerySupportMessagesForThreadResponseItemAuthor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "customer" => Ok(Self::Customer),
            "agent" => Ok(Self::Agent),
            "system" => Ok(Self::System),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QuerySupportMessagesForThreadResponseItemAuthor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Customer => write!(f, "customer"),
            Self::Agent => write!(f, "agent"),
            Self::System => write!(f, "system"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
