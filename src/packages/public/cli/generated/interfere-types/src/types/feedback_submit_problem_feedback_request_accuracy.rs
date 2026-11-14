pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubmitProblemFeedbackRequestAccuracy {
    Correct,
    Partial,
    Wrong,
    Unverifiable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubmitProblemFeedbackRequestAccuracy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Correct => serializer.serialize_str("correct"),
            Self::Partial => serializer.serialize_str("partial"),
            Self::Wrong => serializer.serialize_str("wrong"),
            Self::Unverifiable => serializer.serialize_str("unverifiable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubmitProblemFeedbackRequestAccuracy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "correct" => Ok(Self::Correct),
            "partial" => Ok(Self::Partial),
            "wrong" => Ok(Self::Wrong),
            "unverifiable" => Ok(Self::Unverifiable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubmitProblemFeedbackRequestAccuracy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Correct => write!(f, "correct"),
            Self::Partial => write!(f, "partial"),
            Self::Wrong => write!(f, "wrong"),
            Self::Unverifiable => write!(f, "unverifiable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
