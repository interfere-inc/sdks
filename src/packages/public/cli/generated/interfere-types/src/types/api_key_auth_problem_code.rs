pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApiKeyAuthProblemCode {
    AuthMissing,
    AuthAkInvalid,
    AuthAkSurfaceRequired,
    ReleaseServiceRequired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ApiKeyAuthProblemCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AuthMissing => serializer.serialize_str("AUTH_MISSING"),
            Self::AuthAkInvalid => serializer.serialize_str("AUTH_AK_INVALID"),
            Self::AuthAkSurfaceRequired => serializer.serialize_str("AUTH_AK_SURFACE_REQUIRED"),
            Self::ReleaseServiceRequired => serializer.serialize_str("RELEASE_SERVICE_REQUIRED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ApiKeyAuthProblemCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "AUTH_MISSING" => Ok(Self::AuthMissing),
            "AUTH_AK_INVALID" => Ok(Self::AuthAkInvalid),
            "AUTH_AK_SURFACE_REQUIRED" => Ok(Self::AuthAkSurfaceRequired),
            "RELEASE_SERVICE_REQUIRED" => Ok(Self::ReleaseServiceRequired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ApiKeyAuthProblemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthMissing => write!(f, "AUTH_MISSING"),
            Self::AuthAkInvalid => write!(f, "AUTH_AK_INVALID"),
            Self::AuthAkSurfaceRequired => write!(f, "AUTH_AK_SURFACE_REQUIRED"),
            Self::ReleaseServiceRequired => write!(f, "RELEASE_SERVICE_REQUIRED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
