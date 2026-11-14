pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RespondToActionTimelineRequestArgsResolution {
    Fulfilled,
    Declined,
    Withdrawn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RespondToActionTimelineRequestArgsResolution {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fulfilled => serializer.serialize_str("fulfilled"),
            Self::Declined => serializer.serialize_str("declined"),
            Self::Withdrawn => serializer.serialize_str("withdrawn"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RespondToActionTimelineRequestArgsResolution {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fulfilled" => Ok(Self::Fulfilled),
            "declined" => Ok(Self::Declined),
            "withdrawn" => Ok(Self::Withdrawn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RespondToActionTimelineRequestArgsResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fulfilled => write!(f, "fulfilled"),
            Self::Declined => write!(f, "declined"),
            Self::Withdrawn => write!(f, "withdrawn"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
