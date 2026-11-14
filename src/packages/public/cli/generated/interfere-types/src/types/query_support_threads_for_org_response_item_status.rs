pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QuerySupportThreadsForOrgResponseItemStatus {
    Todo,
    Snoozed,
    Done,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QuerySupportThreadsForOrgResponseItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Todo => serializer.serialize_str("TODO"),
            Self::Snoozed => serializer.serialize_str("SNOOZED"),
            Self::Done => serializer.serialize_str("DONE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QuerySupportThreadsForOrgResponseItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TODO" => Ok(Self::Todo),
            "SNOOZED" => Ok(Self::Snoozed),
            "DONE" => Ok(Self::Done),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QuerySupportThreadsForOrgResponseItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Todo => write!(f, "TODO"),
            Self::Snoozed => write!(f, "SNOOZED"),
            Self::Done => write!(f, "DONE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
