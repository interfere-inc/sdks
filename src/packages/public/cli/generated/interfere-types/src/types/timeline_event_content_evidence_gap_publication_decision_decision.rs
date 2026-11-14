pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimelineEventContentEvidenceGapPublicationDecisionDecision {
    Unsupported,
    InsufficientEvidence,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TimelineEventContentEvidenceGapPublicationDecisionDecision {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Unsupported => serializer.serialize_str("unsupported"),
            Self::InsufficientEvidence => serializer.serialize_str("insufficient-evidence"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TimelineEventContentEvidenceGapPublicationDecisionDecision {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "unsupported" => Ok(Self::Unsupported),
            "insufficient-evidence" => Ok(Self::InsufficientEvidence),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TimelineEventContentEvidenceGapPublicationDecisionDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported => write!(f, "unsupported"),
            Self::InsufficientEvidence => write!(f, "insufficient-evidence"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
