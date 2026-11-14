pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SurfaceEventsTimeseriesResponseUnavailableLinkedServiceSignalsItem {
    Traces,
    Logs,
    Metrics,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SurfaceEventsTimeseriesResponseUnavailableLinkedServiceSignalsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Traces => serializer.serialize_str("traces"),
            Self::Logs => serializer.serialize_str("logs"),
            Self::Metrics => serializer.serialize_str("metrics"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SurfaceEventsTimeseriesResponseUnavailableLinkedServiceSignalsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "traces" => Ok(Self::Traces),
            "logs" => Ok(Self::Logs),
            "metrics" => Ok(Self::Metrics),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SurfaceEventsTimeseriesResponseUnavailableLinkedServiceSignalsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Traces => write!(f, "traces"),
            Self::Logs => write!(f, "logs"),
            Self::Metrics => write!(f, "metrics"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
