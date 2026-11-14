pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateThreadsRequestPriority {
    P0,
    P1,
    P2,
    P3,
    P4,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateThreadsRequestPriority {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::P0 => serializer.serialize_str("P0"),
            Self::P1 => serializer.serialize_str("P1"),
            Self::P2 => serializer.serialize_str("P2"),
            Self::P3 => serializer.serialize_str("P3"),
            Self::P4 => serializer.serialize_str("P4"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateThreadsRequestPriority {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "P0" => Ok(Self::P0),
            "P1" => Ok(Self::P1),
            "P2" => Ok(Self::P2),
            "P3" => Ok(Self::P3),
            "P4" => Ok(Self::P4),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateThreadsRequestPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::P0 => write!(f, "P0"),
            Self::P1 => write!(f, "P1"),
            Self::P2 => write!(f, "P2"),
            Self::P3 => write!(f, "P3"),
            Self::P4 => write!(f, "P4"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
