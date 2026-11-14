pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsInstallationsResponseItemStatusReason {
    ProviderSuspended,
    AuthExpired,
    MissingPermissions,
    SetupIncomplete,
    AlreadyConnectedElsewhere,
    UnknownError,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryIntegrationsInstallationsResponseItemStatusReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ProviderSuspended => serializer.serialize_str("provider_suspended"),
            Self::AuthExpired => serializer.serialize_str("auth_expired"),
            Self::MissingPermissions => serializer.serialize_str("missing_permissions"),
            Self::SetupIncomplete => serializer.serialize_str("setup_incomplete"),
            Self::AlreadyConnectedElsewhere => serializer.serialize_str("already_connected_elsewhere"),
            Self::UnknownError => serializer.serialize_str("unknown_error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryIntegrationsInstallationsResponseItemStatusReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "provider_suspended" => Ok(Self::ProviderSuspended),
            "auth_expired" => Ok(Self::AuthExpired),
            "missing_permissions" => Ok(Self::MissingPermissions),
            "setup_incomplete" => Ok(Self::SetupIncomplete),
            "already_connected_elsewhere" => Ok(Self::AlreadyConnectedElsewhere),
            "unknown_error" => Ok(Self::UnknownError),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryIntegrationsInstallationsResponseItemStatusReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProviderSuspended => write!(f, "provider_suspended"),
            Self::AuthExpired => write!(f, "auth_expired"),
            Self::MissingPermissions => write!(f, "missing_permissions"),
            Self::SetupIncomplete => write!(f, "setup_incomplete"),
            Self::AlreadyConnectedElsewhere => write!(f, "already_connected_elsewhere"),
            Self::UnknownError => write!(f, "unknown_error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
