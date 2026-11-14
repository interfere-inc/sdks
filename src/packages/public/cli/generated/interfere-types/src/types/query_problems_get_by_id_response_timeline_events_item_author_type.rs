pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemAuthorType {
    Agent,
    User,
    Integration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsGetByIdResponseTimelineEventsItemAuthorType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Agent => serializer.serialize_str("AGENT"),
            Self::User => serializer.serialize_str("USER"),
            Self::Integration => serializer.serialize_str("INTEGRATION"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsGetByIdResponseTimelineEventsItemAuthorType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "AGENT" => Ok(Self::Agent),
            "USER" => Ok(Self::User),
            "INTEGRATION" => Ok(Self::Integration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemAuthorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Agent => write!(f, "AGENT"),
            Self::User => write!(f, "USER"),
            Self::Integration => write!(f, "INTEGRATION"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
