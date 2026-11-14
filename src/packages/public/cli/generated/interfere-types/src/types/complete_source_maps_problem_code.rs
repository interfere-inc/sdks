pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompleteSourceMapsProblemCode {
    ReleaseNotFound,
    ReleaseUploadInvalid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CompleteSourceMapsProblemCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ReleaseNotFound => serializer.serialize_str("RELEASE_NOT_FOUND"),
            Self::ReleaseUploadInvalid => serializer.serialize_str("RELEASE_UPLOAD_INVALID"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CompleteSourceMapsProblemCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "RELEASE_NOT_FOUND" => Ok(Self::ReleaseNotFound),
            "RELEASE_UPLOAD_INVALID" => Ok(Self::ReleaseUploadInvalid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CompleteSourceMapsProblemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReleaseNotFound => write!(f, "RELEASE_NOT_FOUND"),
            Self::ReleaseUploadInvalid => write!(f, "RELEASE_UPLOAD_INVALID"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
