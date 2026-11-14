pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryFindingsListByProblemResponseItemStatus {
    New,
    Investigating,
    Promoted,
    NeedsAttention,
    Suppressed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryFindingsListByProblemResponseItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::New => serializer.serialize_str("NEW"),
            Self::Investigating => serializer.serialize_str("INVESTIGATING"),
            Self::Promoted => serializer.serialize_str("PROMOTED"),
            Self::NeedsAttention => serializer.serialize_str("NEEDS_ATTENTION"),
            Self::Suppressed => serializer.serialize_str("SUPPRESSED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryFindingsListByProblemResponseItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NEW" => Ok(Self::New),
            "INVESTIGATING" => Ok(Self::Investigating),
            "PROMOTED" => Ok(Self::Promoted),
            "NEEDS_ATTENTION" => Ok(Self::NeedsAttention),
            "SUPPRESSED" => Ok(Self::Suppressed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryFindingsListByProblemResponseItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::New => write!(f, "NEW"),
            Self::Investigating => write!(f, "INVESTIGATING"),
            Self::Promoted => write!(f, "PROMOTED"),
            Self::NeedsAttention => write!(f, "NEEDS_ATTENTION"),
            Self::Suppressed => write!(f, "SUPPRESSED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
