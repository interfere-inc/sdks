pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindActionKind {
    TextInput,
    SingleSelect,
    MultiSelect,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindActionKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TextInput => serializer.serialize_str("text-input"),
            Self::SingleSelect => serializer.serialize_str("single-select"),
            Self::MultiSelect => serializer.serialize_str("multi-select"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindActionKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text-input" => Ok(Self::TextInput),
            "single-select" => Ok(Self::SingleSelect),
            "multi-select" => Ok(Self::MultiSelect),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemRequestContractCompletionActionKindActionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TextInput => write!(f, "text-input"),
            Self::SingleSelect => write!(f, "single-select"),
            Self::MultiSelect => write!(f, "multi-select"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
