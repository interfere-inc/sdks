pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SurfaceActivityResponseValueStatus {
    Down,
    Receiving,
    Stale,
    Outdated,
    SdkSilent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SurfaceActivityResponseValueStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Down => serializer.serialize_str("down"),
            Self::Receiving => serializer.serialize_str("receiving"),
            Self::Stale => serializer.serialize_str("stale"),
            Self::Outdated => serializer.serialize_str("outdated"),
            Self::SdkSilent => serializer.serialize_str("sdk-silent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SurfaceActivityResponseValueStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "down" => Ok(Self::Down),
            "receiving" => Ok(Self::Receiving),
            "stale" => Ok(Self::Stale),
            "outdated" => Ok(Self::Outdated),
            "sdk-silent" => Ok(Self::SdkSilent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SurfaceActivityResponseValueStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Down => write!(f, "down"),
            Self::Receiving => write!(f, "receiving"),
            Self::Stale => write!(f, "stale"),
            Self::Outdated => write!(f, "outdated"),
            Self::SdkSilent => write!(f, "sdk-silent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
