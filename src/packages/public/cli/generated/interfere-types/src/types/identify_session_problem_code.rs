pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdentifySessionProblemCode {
    IdentitySignatureInvalid,
    IngestInfraUnavailable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for IdentifySessionProblemCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::IdentitySignatureInvalid => serializer.serialize_str("IDENTITY_SIGNATURE_INVALID"),
            Self::IngestInfraUnavailable => serializer.serialize_str("INGEST_INFRA_UNAVAILABLE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for IdentifySessionProblemCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "IDENTITY_SIGNATURE_INVALID" => Ok(Self::IdentitySignatureInvalid),
            "INGEST_INFRA_UNAVAILABLE" => Ok(Self::IngestInfraUnavailable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for IdentifySessionProblemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentitySignatureInvalid => write!(f, "IDENTITY_SIGNATURE_INVALID"),
            Self::IngestInfraUnavailable => write!(f, "INGEST_INFRA_UNAVAILABLE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
