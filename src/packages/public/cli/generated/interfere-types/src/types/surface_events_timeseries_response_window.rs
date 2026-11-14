pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SurfaceEventsTimeseriesResponseWindow {
    FourH,
    EightH,
    TwelveH,
    TwentyFourH,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SurfaceEventsTimeseriesResponseWindow {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FourH => serializer.serialize_str("4h"),
            Self::EightH => serializer.serialize_str("8h"),
            Self::TwelveH => serializer.serialize_str("12h"),
            Self::TwentyFourH => serializer.serialize_str("24h"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SurfaceEventsTimeseriesResponseWindow {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "4h" => Ok(Self::FourH),
            "8h" => Ok(Self::EightH),
            "12h" => Ok(Self::TwelveH),
            "24h" => Ok(Self::TwentyFourH),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SurfaceEventsTimeseriesResponseWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FourH => write!(f, "4h"),
            Self::EightH => write!(f, "8h"),
            Self::TwelveH => write!(f, "12h"),
            Self::TwentyFourH => write!(f, "24h"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
