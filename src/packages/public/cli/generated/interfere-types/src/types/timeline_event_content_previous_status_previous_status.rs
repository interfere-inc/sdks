pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimelineEventContentPreviousStatusPreviousStatus {
    Draft,
    Detected,
    Reasoning,
    Failed,
    Active,
    InRecovery,
    NeedsAttention,
    Resolved,
    Dismissed,
    NotAProblem,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TimelineEventContentPreviousStatusPreviousStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("DRAFT"),
            Self::Detected => serializer.serialize_str("DETECTED"),
            Self::Reasoning => serializer.serialize_str("REASONING"),
            Self::Failed => serializer.serialize_str("FAILED"),
            Self::Active => serializer.serialize_str("ACTIVE"),
            Self::InRecovery => serializer.serialize_str("IN_RECOVERY"),
            Self::NeedsAttention => serializer.serialize_str("NEEDS_ATTENTION"),
            Self::Resolved => serializer.serialize_str("RESOLVED"),
            Self::Dismissed => serializer.serialize_str("DISMISSED"),
            Self::NotAProblem => serializer.serialize_str("NOT_A_PROBLEM"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TimelineEventContentPreviousStatusPreviousStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DRAFT" => Ok(Self::Draft),
            "DETECTED" => Ok(Self::Detected),
            "REASONING" => Ok(Self::Reasoning),
            "FAILED" => Ok(Self::Failed),
            "ACTIVE" => Ok(Self::Active),
            "IN_RECOVERY" => Ok(Self::InRecovery),
            "NEEDS_ATTENTION" => Ok(Self::NeedsAttention),
            "RESOLVED" => Ok(Self::Resolved),
            "DISMISSED" => Ok(Self::Dismissed),
            "NOT_A_PROBLEM" => Ok(Self::NotAProblem),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TimelineEventContentPreviousStatusPreviousStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "DRAFT"),
            Self::Detected => write!(f, "DETECTED"),
            Self::Reasoning => write!(f, "REASONING"),
            Self::Failed => write!(f, "FAILED"),
            Self::Active => write!(f, "ACTIVE"),
            Self::InRecovery => write!(f, "IN_RECOVERY"),
            Self::NeedsAttention => write!(f, "NEEDS_ATTENTION"),
            Self::Resolved => write!(f, "RESOLVED"),
            Self::Dismissed => write!(f, "DISMISSED"),
            Self::NotAProblem => write!(f, "NOT_A_PROBLEM"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
