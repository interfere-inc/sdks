pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What the feedback is about: `feature` requests a product change, `bug` reports a defect in Interfere, `integration` requests an integration, and `missed` reports a problem in the workspace's own product that Interfere failed to detect.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubmitWorkspaceFeedbackRequestKind {
    Feature,
    Bug,
    Integration,
    Missed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubmitWorkspaceFeedbackRequestKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Feature => serializer.serialize_str("feature"),
            Self::Bug => serializer.serialize_str("bug"),
            Self::Integration => serializer.serialize_str("integration"),
            Self::Missed => serializer.serialize_str("missed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubmitWorkspaceFeedbackRequestKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "feature" => Ok(Self::Feature),
            "bug" => Ok(Self::Bug),
            "integration" => Ok(Self::Integration),
            "missed" => Ok(Self::Missed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubmitWorkspaceFeedbackRequestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Feature => write!(f, "feature"),
            Self::Bug => write!(f, "bug"),
            Self::Integration => write!(f, "integration"),
            Self::Missed => write!(f, "missed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
