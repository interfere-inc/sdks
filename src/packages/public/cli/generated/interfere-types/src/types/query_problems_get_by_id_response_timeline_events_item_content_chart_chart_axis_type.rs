pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisType {
    Area,
    Bar,
    Line,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Area => serializer.serialize_str("area"),
            Self::Bar => serializer.serialize_str("bar"),
            Self::Line => serializer.serialize_str("line"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "area" => Ok(Self::Area),
            "bar" => Ok(Self::Bar),
            "line" => Ok(Self::Line),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Area => write!(f, "area"),
            Self::Bar => write!(f, "bar"),
            Self::Line => write!(f, "line"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
