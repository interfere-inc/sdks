pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsVercelInstallationResponseProviderMetadataTwoFlavor {
    GithubCom,
    GheCloud,
    GheServer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryIntegrationsVercelInstallationResponseProviderMetadataTwoFlavor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::GithubCom => serializer.serialize_str("github_com"),
            Self::GheCloud => serializer.serialize_str("ghe_cloud"),
            Self::GheServer => serializer.serialize_str("ghe_server"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryIntegrationsVercelInstallationResponseProviderMetadataTwoFlavor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "github_com" => Ok(Self::GithubCom),
            "ghe_cloud" => Ok(Self::GheCloud),
            "ghe_server" => Ok(Self::GheServer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryIntegrationsVercelInstallationResponseProviderMetadataTwoFlavor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GithubCom => write!(f, "github_com"),
            Self::GheCloud => write!(f, "ghe_cloud"),
            Self::GheServer => write!(f, "ghe_server"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
