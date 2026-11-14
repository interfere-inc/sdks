pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignSourceMapsProblemCode {
    ReleaseNotFound,
    ReleaseSignEmpty,
    ReleaseSignTooManyFiles,
    ReleaseSignFileTooLarge,
    ReleaseSignDuplicatePath,
    ReleaseUploadInvalid,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SignSourceMapsProblemCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ReleaseNotFound => serializer.serialize_str("RELEASE_NOT_FOUND"),
            Self::ReleaseSignEmpty => serializer.serialize_str("RELEASE_SIGN_EMPTY"),
            Self::ReleaseSignTooManyFiles => serializer.serialize_str("RELEASE_SIGN_TOO_MANY_FILES"),
            Self::ReleaseSignFileTooLarge => serializer.serialize_str("RELEASE_SIGN_FILE_TOO_LARGE"),
            Self::ReleaseSignDuplicatePath => serializer.serialize_str("RELEASE_SIGN_DUPLICATE_PATH"),
            Self::ReleaseUploadInvalid => serializer.serialize_str("RELEASE_UPLOAD_INVALID"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SignSourceMapsProblemCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "RELEASE_NOT_FOUND" => Ok(Self::ReleaseNotFound),
            "RELEASE_SIGN_EMPTY" => Ok(Self::ReleaseSignEmpty),
            "RELEASE_SIGN_TOO_MANY_FILES" => Ok(Self::ReleaseSignTooManyFiles),
            "RELEASE_SIGN_FILE_TOO_LARGE" => Ok(Self::ReleaseSignFileTooLarge),
            "RELEASE_SIGN_DUPLICATE_PATH" => Ok(Self::ReleaseSignDuplicatePath),
            "RELEASE_UPLOAD_INVALID" => Ok(Self::ReleaseUploadInvalid),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SignSourceMapsProblemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReleaseNotFound => write!(f, "RELEASE_NOT_FOUND"),
            Self::ReleaseSignEmpty => write!(f, "RELEASE_SIGN_EMPTY"),
            Self::ReleaseSignTooManyFiles => write!(f, "RELEASE_SIGN_TOO_MANY_FILES"),
            Self::ReleaseSignFileTooLarge => write!(f, "RELEASE_SIGN_FILE_TOO_LARGE"),
            Self::ReleaseSignDuplicatePath => write!(f, "RELEASE_SIGN_DUPLICATE_PATH"),
            Self::ReleaseUploadInvalid => write!(f, "RELEASE_UPLOAD_INVALID"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
