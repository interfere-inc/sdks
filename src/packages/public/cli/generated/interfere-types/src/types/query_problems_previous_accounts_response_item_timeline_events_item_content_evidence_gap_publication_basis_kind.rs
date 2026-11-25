pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisKind {
    Note,
    RootCause,
    Fix,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Note => serializer.serialize_str("note"),
            Self::RootCause => serializer.serialize_str("root-cause"),
            Self::Fix => serializer.serialize_str("fix"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "note" => Ok(Self::Note),
            "root-cause" => Ok(Self::RootCause),
            "fix" => Ok(Self::Fix),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentEvidenceGapPublicationBasisKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Note => write!(f, "note"),
            Self::RootCause => write!(f, "root-cause"),
            Self::Fix => write!(f, "fix"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
