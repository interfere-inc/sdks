pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCause {
    Code,
    Dependency,
    Infrastructure,
    External,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCause {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Code => serializer.serialize_str("code"),
            Self::Dependency => serializer.serialize_str("dependency"),
            Self::Infrastructure => serializer.serialize_str("infrastructure"),
            Self::External => serializer.serialize_str("external"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCause {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "code" => Ok(Self::Code),
            "dependency" => Ok(Self::Dependency),
            "infrastructure" => Ok(Self::Infrastructure),
            "external" => Ok(Self::External),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Code => write!(f, "code"),
            Self::Dependency => write!(f, "dependency"),
            Self::Infrastructure => write!(f, "infrastructure"),
            Self::External => write!(f, "external"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
