pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimelineEventContentTwelveCauseCorrectionReason {
    WrongMechanism,
    WrongLocation,
    MissingDetail,
    Wording,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TimelineEventContentTwelveCauseCorrectionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WrongMechanism => serializer.serialize_str("wrong-mechanism"),
            Self::WrongLocation => serializer.serialize_str("wrong-location"),
            Self::MissingDetail => serializer.serialize_str("missing-detail"),
            Self::Wording => serializer.serialize_str("wording"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TimelineEventContentTwelveCauseCorrectionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "wrong-mechanism" => Ok(Self::WrongMechanism),
            "wrong-location" => Ok(Self::WrongLocation),
            "missing-detail" => Ok(Self::MissingDetail),
            "wording" => Ok(Self::Wording),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TimelineEventContentTwelveCauseCorrectionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongMechanism => write!(f, "wrong-mechanism"),
            Self::WrongLocation => write!(f, "wrong-location"),
            Self::MissingDetail => write!(f, "missing-detail"),
            Self::Wording => write!(f, "wording"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
