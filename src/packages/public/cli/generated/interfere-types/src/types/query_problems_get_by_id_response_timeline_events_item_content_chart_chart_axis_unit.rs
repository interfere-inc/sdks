pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisUnit {
    Bytes,
    Count,
    Duration,
    Percent,
    Score,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bytes => serializer.serialize_str("bytes"),
            Self::Count => serializer.serialize_str("count"),
            Self::Duration => serializer.serialize_str("duration"),
            Self::Percent => serializer.serialize_str("percent"),
            Self::Score => serializer.serialize_str("score"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bytes" => Ok(Self::Bytes),
            "count" => Ok(Self::Count),
            "duration" => Ok(Self::Duration),
            "percent" => Ok(Self::Percent),
            "score" => Ok(Self::Score),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bytes => write!(f, "bytes"),
            Self::Count => write!(f, "count"),
            Self::Duration => write!(f, "duration"),
            Self::Percent => write!(f, "percent"),
            Self::Score => write!(f, "score"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
