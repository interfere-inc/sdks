pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero {
    SourceMap,
    SourceAccess,
    Release,
    Replay,
    Capture,
    UserAttribution,
    AppData,
    RecoveryWindow,
    UpstreamService,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SourceMap => serializer.serialize_str("source-map"),
            Self::SourceAccess => serializer.serialize_str("source-access"),
            Self::Release => serializer.serialize_str("release"),
            Self::Replay => serializer.serialize_str("replay"),
            Self::Capture => serializer.serialize_str("capture"),
            Self::UserAttribution => serializer.serialize_str("user-attribution"),
            Self::AppData => serializer.serialize_str("app-data"),
            Self::RecoveryWindow => serializer.serialize_str("recovery-window"),
            Self::UpstreamService => serializer.serialize_str("upstream-service"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "source-map" => Ok(Self::SourceMap),
            "source-access" => Ok(Self::SourceAccess),
            "release" => Ok(Self::Release),
            "replay" => Ok(Self::Replay),
            "capture" => Ok(Self::Capture),
            "user-attribution" => Ok(Self::UserAttribution),
            "app-data" => Ok(Self::AppData),
            "recovery-window" => Ok(Self::RecoveryWindow),
            "upstream-service" => Ok(Self::UpstreamService),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TimelineEventContentActionKindEvidenceStatusReceiptRequiredEvidenceKindZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceMap => write!(f, "source-map"),
            Self::SourceAccess => write!(f, "source-access"),
            Self::Release => write!(f, "release"),
            Self::Replay => write!(f, "replay"),
            Self::Capture => write!(f, "capture"),
            Self::UserAttribution => write!(f, "user-attribution"),
            Self::AppData => write!(f, "app-data"),
            Self::RecoveryWindow => write!(f, "recovery-window"),
            Self::UpstreamService => write!(f, "upstream-service"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
