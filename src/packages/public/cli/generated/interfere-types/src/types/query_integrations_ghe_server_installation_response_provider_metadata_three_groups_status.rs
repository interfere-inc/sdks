pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeGroupsStatus {
    Synced,
    MissingPermission,
    UnsupportedAccount,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeGroupsStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Synced => serializer.serialize_str("synced"),
            Self::MissingPermission => serializer.serialize_str("missing_permission"),
            Self::UnsupportedAccount => serializer.serialize_str("unsupported_account"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeGroupsStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "synced" => Ok(Self::Synced),
            "missing_permission" => Ok(Self::MissingPermission),
            "unsupported_account" => Ok(Self::UnsupportedAccount),
            "failed" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeGroupsStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Synced => write!(f, "synced"),
            Self::MissingPermission => write!(f, "missing_permission"),
            Self::UnsupportedAccount => write!(f, "unsupported_account"),
            Self::Failed => write!(f, "failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
