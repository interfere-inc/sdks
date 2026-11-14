pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindActionKind {
    AccessRequest,
    EvidenceGap,
    McpConnection,
    SingleSelect,
    MultiSelect,
    TextInput,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindActionKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AccessRequest => serializer.serialize_str("access-request"),
            Self::EvidenceGap => serializer.serialize_str("evidence-gap"),
            Self::McpConnection => serializer.serialize_str("mcp-connection"),
            Self::SingleSelect => serializer.serialize_str("single-select"),
            Self::MultiSelect => serializer.serialize_str("multi-select"),
            Self::TextInput => serializer.serialize_str("text-input"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindActionKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "access-request" => Ok(Self::AccessRequest),
            "evidence-gap" => Ok(Self::EvidenceGap),
            "mcp-connection" => Ok(Self::McpConnection),
            "single-select" => Ok(Self::SingleSelect),
            "multi-select" => Ok(Self::MultiSelect),
            "text-input" => Ok(Self::TextInput),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionKindActionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccessRequest => write!(f, "access-request"),
            Self::EvidenceGap => write!(f, "evidence-gap"),
            Self::McpConnection => write!(f, "mcp-connection"),
            Self::SingleSelect => write!(f, "single-select"),
            Self::MultiSelect => write!(f, "multi-select"),
            Self::TextInput => write!(f, "text-input"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
