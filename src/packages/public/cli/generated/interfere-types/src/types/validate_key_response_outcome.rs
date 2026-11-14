pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidateKeyResponseOutcome {
    Valid,
    Revoked,
    Rejected,
    MissingScope,
    SurfaceRequired,
    WrongWorkspace,
    WrongRegion,
    WrongKind,
    NotFound,
    Malformed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ValidateKeyResponseOutcome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Valid => serializer.serialize_str("valid"),
            Self::Revoked => serializer.serialize_str("revoked"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::MissingScope => serializer.serialize_str("missing_scope"),
            Self::SurfaceRequired => serializer.serialize_str("surface_required"),
            Self::WrongWorkspace => serializer.serialize_str("wrong_workspace"),
            Self::WrongRegion => serializer.serialize_str("wrong_region"),
            Self::WrongKind => serializer.serialize_str("wrong_kind"),
            Self::NotFound => serializer.serialize_str("not_found"),
            Self::Malformed => serializer.serialize_str("malformed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ValidateKeyResponseOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "valid" => Ok(Self::Valid),
            "revoked" => Ok(Self::Revoked),
            "rejected" => Ok(Self::Rejected),
            "missing_scope" => Ok(Self::MissingScope),
            "surface_required" => Ok(Self::SurfaceRequired),
            "wrong_workspace" => Ok(Self::WrongWorkspace),
            "wrong_region" => Ok(Self::WrongRegion),
            "wrong_kind" => Ok(Self::WrongKind),
            "not_found" => Ok(Self::NotFound),
            "malformed" => Ok(Self::Malformed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ValidateKeyResponseOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid => write!(f, "valid"),
            Self::Revoked => write!(f, "revoked"),
            Self::Rejected => write!(f, "rejected"),
            Self::MissingScope => write!(f, "missing_scope"),
            Self::SurfaceRequired => write!(f, "surface_required"),
            Self::WrongWorkspace => write!(f, "wrong_workspace"),
            Self::WrongRegion => write!(f, "wrong_region"),
            Self::WrongKind => write!(f, "wrong_kind"),
            Self::NotFound => write!(f, "not_found"),
            Self::Malformed => write!(f, "malformed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
