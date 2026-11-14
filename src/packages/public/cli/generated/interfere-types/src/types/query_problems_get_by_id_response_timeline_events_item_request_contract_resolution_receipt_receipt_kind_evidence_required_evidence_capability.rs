pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability {
    NativeLogs,
    Repository,
    Mcp,
    Replay,
    Release,
    SourceMap,
    UserAttribution,
    HumanAnswer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NativeLogs => serializer.serialize_str("native-logs"),
            Self::Repository => serializer.serialize_str("repository"),
            Self::Mcp => serializer.serialize_str("mcp"),
            Self::Replay => serializer.serialize_str("replay"),
            Self::Release => serializer.serialize_str("release"),
            Self::SourceMap => serializer.serialize_str("source-map"),
            Self::UserAttribution => serializer.serialize_str("user-attribution"),
            Self::HumanAnswer => serializer.serialize_str("human-answer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "native-logs" => Ok(Self::NativeLogs),
            "repository" => Ok(Self::Repository),
            "mcp" => Ok(Self::Mcp),
            "replay" => Ok(Self::Replay),
            "release" => Ok(Self::Release),
            "source-map" => Ok(Self::SourceMap),
            "user-attribution" => Ok(Self::UserAttribution),
            "human-answer" => Ok(Self::HumanAnswer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemRequestContractResolutionReceiptReceiptKindEvidenceRequiredEvidenceCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NativeLogs => write!(f, "native-logs"),
            Self::Repository => write!(f, "repository"),
            Self::Mcp => write!(f, "mcp"),
            Self::Replay => write!(f, "replay"),
            Self::Release => write!(f, "release"),
            Self::SourceMap => write!(f, "source-map"),
            Self::UserAttribution => write!(f, "user-attribution"),
            Self::HumanAnswer => write!(f, "human-answer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
