pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceMapPublicationStatusDiagnosticsLastFailureKind {
    Timeout,
    Aborted,
    Error,
    NonError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SourceMapPublicationStatusDiagnosticsLastFailureKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Timeout => serializer.serialize_str("timeout"),
            Self::Aborted => serializer.serialize_str("aborted"),
            Self::Error => serializer.serialize_str("error"),
            Self::NonError => serializer.serialize_str("non-error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SourceMapPublicationStatusDiagnosticsLastFailureKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "timeout" => Ok(Self::Timeout),
            "aborted" => Ok(Self::Aborted),
            "error" => Ok(Self::Error),
            "non-error" => Ok(Self::NonError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SourceMapPublicationStatusDiagnosticsLastFailureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout => write!(f, "timeout"),
            Self::Aborted => write!(f, "aborted"),
            Self::Error => write!(f, "error"),
            Self::NonError => write!(f, "non-error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
