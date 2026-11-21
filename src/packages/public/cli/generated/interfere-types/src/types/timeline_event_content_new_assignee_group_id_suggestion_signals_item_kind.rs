pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemKind {
    Store,
    History,
    People,
    Codeowners,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Store => serializer.serialize_str("store"),
            Self::History => serializer.serialize_str("history"),
            Self::People => serializer.serialize_str("people"),
            Self::Codeowners => serializer.serialize_str("codeowners"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "store" => Ok(Self::Store),
            "history" => Ok(Self::History),
            "people" => Ok(Self::People),
            "codeowners" => Ok(Self::Codeowners),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Store => write!(f, "store"),
            Self::History => write!(f, "history"),
            Self::People => write!(f, "people"),
            Self::Codeowners => write!(f, "codeowners"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
