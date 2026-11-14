pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ReleaseBuildResult
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReleaseBuildResult {
    Ok,
    ApiKeyWrongPrefix,
    ApiKeyUnreadable,
    ApiKeyRejected,
    ApiKeyScopeMismatch,
    ApiKeySurfaceRequired,
    PipelineFailed,
    NoSourceMaps,
    ClientMapsUnmapped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReleaseBuildResult {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ok => serializer.serialize_str("ok"),
            Self::ApiKeyWrongPrefix => serializer.serialize_str("api_key_wrong_prefix"),
            Self::ApiKeyUnreadable => serializer.serialize_str("api_key_unreadable"),
            Self::ApiKeyRejected => serializer.serialize_str("api_key_rejected"),
            Self::ApiKeyScopeMismatch => serializer.serialize_str("api_key_scope_mismatch"),
            Self::ApiKeySurfaceRequired => serializer.serialize_str("api_key_surface_required"),
            Self::PipelineFailed => serializer.serialize_str("pipeline_failed"),
            Self::NoSourceMaps => serializer.serialize_str("no_source_maps"),
            Self::ClientMapsUnmapped => serializer.serialize_str("client_maps_unmapped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReleaseBuildResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ok" => Ok(Self::Ok),
            "api_key_wrong_prefix" => Ok(Self::ApiKeyWrongPrefix),
            "api_key_unreadable" => Ok(Self::ApiKeyUnreadable),
            "api_key_rejected" => Ok(Self::ApiKeyRejected),
            "api_key_scope_mismatch" => Ok(Self::ApiKeyScopeMismatch),
            "api_key_surface_required" => Ok(Self::ApiKeySurfaceRequired),
            "pipeline_failed" => Ok(Self::PipelineFailed),
            "no_source_maps" => Ok(Self::NoSourceMaps),
            "client_maps_unmapped" => Ok(Self::ClientMapsUnmapped),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReleaseBuildResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "ok"),
            Self::ApiKeyWrongPrefix => write!(f, "api_key_wrong_prefix"),
            Self::ApiKeyUnreadable => write!(f, "api_key_unreadable"),
            Self::ApiKeyRejected => write!(f, "api_key_rejected"),
            Self::ApiKeyScopeMismatch => write!(f, "api_key_scope_mismatch"),
            Self::ApiKeySurfaceRequired => write!(f, "api_key_surface_required"),
            Self::PipelineFailed => write!(f, "pipeline_failed"),
            Self::NoSourceMaps => write!(f, "no_source_maps"),
            Self::ClientMapsUnmapped => write!(f, "client_maps_unmapped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
