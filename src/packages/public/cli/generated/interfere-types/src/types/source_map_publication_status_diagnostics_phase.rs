pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceMapPublicationStatusDiagnosticsPhase {
    Files,
    Manifest,
    Release,
    Continuation,
    Completion,
    Complete,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SourceMapPublicationStatusDiagnosticsPhase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Files => serializer.serialize_str("files"),
            Self::Manifest => serializer.serialize_str("manifest"),
            Self::Release => serializer.serialize_str("release"),
            Self::Continuation => serializer.serialize_str("continuation"),
            Self::Completion => serializer.serialize_str("completion"),
            Self::Complete => serializer.serialize_str("complete"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SourceMapPublicationStatusDiagnosticsPhase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "files" => Ok(Self::Files),
            "manifest" => Ok(Self::Manifest),
            "release" => Ok(Self::Release),
            "continuation" => Ok(Self::Continuation),
            "completion" => Ok(Self::Completion),
            "complete" => Ok(Self::Complete),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SourceMapPublicationStatusDiagnosticsPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Files => write!(f, "files"),
            Self::Manifest => write!(f, "manifest"),
            Self::Release => write!(f, "release"),
            Self::Continuation => write!(f, "continuation"),
            Self::Completion => write!(f, "completion"),
            Self::Complete => write!(f, "complete"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
