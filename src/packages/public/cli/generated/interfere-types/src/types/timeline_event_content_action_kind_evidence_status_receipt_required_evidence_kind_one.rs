pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne {
    Logs,
    Configuration,
    Answer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Logs => serializer.serialize_str("logs"),
            Self::Configuration => serializer.serialize_str("configuration"),
            Self::Answer => serializer.serialize_str("answer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "logs" => Ok(Self::Logs),
            "configuration" => Ok(Self::Configuration),
            "answer" => Ok(Self::Answer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Logs => write!(f, "logs"),
            Self::Configuration => write!(f, "configuration"),
            Self::Answer => write!(f, "answer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
