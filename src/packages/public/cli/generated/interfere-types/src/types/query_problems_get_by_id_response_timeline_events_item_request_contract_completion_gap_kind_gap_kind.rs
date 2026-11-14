pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindGapKind {
    Release,
    Replay,
    SourceMap,
    UserAttribution,
    UpstreamService,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindGapKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Release => serializer.serialize_str("release"),
            Self::Replay => serializer.serialize_str("replay"),
            Self::SourceMap => serializer.serialize_str("source-map"),
            Self::UserAttribution => serializer.serialize_str("user-attribution"),
            Self::UpstreamService => serializer.serialize_str("upstream-service"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindGapKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "release" => Ok(Self::Release),
            "replay" => Ok(Self::Replay),
            "source-map" => Ok(Self::SourceMap),
            "user-attribution" => Ok(Self::UserAttribution),
            "upstream-service" => Ok(Self::UpstreamService),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractCompletionGapKindGapKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Release => write!(f, "release"),
            Self::Replay => write!(f, "replay"),
            Self::SourceMap => write!(f, "source-map"),
            Self::UserAttribution => write!(f, "user-attribution"),
            Self::UpstreamService => write!(f, "upstream-service"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
